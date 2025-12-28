macro_rules! deps {
    () => {
        GenericPredicates!();
        TypingMode!();
        HirDatabase!();
    };
}

macro_rules! specializes_query {
    () => {
        deps!();
        # [doc = " Is `specializing_impl_def_id` a specialization of `parent_impl_def_id`?"] # [doc = ""] # [doc = " For every type that could apply to `specializing_impl_def_id`, we prove that"] # [doc = " the `parent_impl_def_id` also applies (i.e. it has a valid impl header and"] # [doc = " its where-clauses hold)."] # [doc = ""] # [doc = " For the purposes of const traits, we also check that the specializing"] # [doc = " impl is not more restrictive than the parent impl. That is, if the"] # [doc = " `parent_impl_def_id` is a const impl (conditionally based off of some `[const]`"] # [doc = " bounds), then `specializing_impl_def_id` must also be const for the same"] # [doc = " set of types."] # [salsa :: tracked (cycle_result = specializes_query_cycle)] fn specializes_query (db : & dyn HirDatabase , specializing_impl_def_id : ImplId , parent_impl_def_id : ImplId ,) -> bool { let trait_env = db . trait_environment (specializing_impl_def_id . into ()) ; let interner = DbInterner :: new_with (db , Some (trait_env . krate) , trait_env . block) ; let specializing_impl_signature = db . impl_signature (specializing_impl_def_id) ; let parent_impl_signature = db . impl_signature (parent_impl_def_id) ; if specializing_impl_signature . is_negative () != parent_impl_signature . is_negative () { return false ; } let param_env = trait_env . env ; let infcx = interner . infer_ctxt () . build (TypingMode :: non_body_analysis ()) ; let specializing_impl_trait_ref = db . impl_trait (specializing_impl_def_id) . unwrap () . instantiate_identity () ; let cause = & ObligationCause :: dummy () ; debug ! ("fulfill_implication({:?}, trait_ref={:?} |- {:?} applies)" , param_env , specializing_impl_trait_ref , parent_impl_def_id) ; let mut ocx = ObligationCtxt :: new (& infcx) ; let parent_args = infcx . fresh_args_for_item (parent_impl_def_id . into ()) ; let parent_impl_trait_ref = db . impl_trait (parent_impl_def_id) . expect ("expected source impl to be a trait impl") . instantiate (interner , parent_args) ; let Ok (()) = ocx . eq (cause , param_env , specializing_impl_trait_ref , parent_impl_trait_ref) else { return false ; } ; ocx . register_obligations (clauses_as_obligations (GenericPredicates :: query_all (db , parent_impl_def_id . into ()) . iter_instantiated_copied (interner , parent_args . as_slice ()) , cause . clone () , param_env ,)) ; let errors = ocx . evaluate_obligations_error_on_ambiguity () ; if ! errors . is_empty () { debug ! ("fulfill_implication: for impls on {:?} and {:?}, \
                 could not fulfill: {:?} given {:?}" , specializing_impl_trait_ref , parent_impl_trait_ref , errors , param_env) ; return false ; } debug ! ("fulfill_implication: an impl for {:?} specializes {:?}" , specializing_impl_trait_ref , parent_impl_trait_ref) ; true }
    };
}

specializes_query!();
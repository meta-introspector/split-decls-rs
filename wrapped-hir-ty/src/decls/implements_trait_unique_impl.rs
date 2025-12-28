macro_rules! deps {
    () => {
        TypingMode!();
        HirDatabase!();
        TraitEnvironment!();
    };
}

macro_rules! implements_trait_unique_impl {
    () => {
        deps!();
        fn implements_trait_unique_impl < 'db > (db : & 'db dyn HirDatabase , env : Arc < TraitEnvironment < 'db > > , trait_ : TraitId , create_args : & mut dyn FnMut (& InferCtxt < 'db >) -> GenericArgs < 'db > ,) -> bool { let interner = DbInterner :: new_with (db , Some (env . krate) , env . block) ; let infcx = interner . infer_ctxt () . build (TypingMode :: non_body_analysis ()) ; let args = create_args (& infcx) ; let trait_ref = rustc_type_ir :: TraitRef :: new_from_args (interner , trait_ . into () , args) ; let goal = Goal :: new (interner , env . env , trait_ref) ; let result = crate :: traits :: next_trait_solve_in_ctxt (& infcx , goal) ; matches ! (result , Ok ((_ , Certainty :: Yes))) }
    };
}

implements_trait_unique_impl!()
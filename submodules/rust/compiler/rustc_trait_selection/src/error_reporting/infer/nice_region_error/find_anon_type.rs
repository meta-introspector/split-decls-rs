mkuse!{use core :: ops :: ControlFlow ;}
mkuse!{use rustc_hir :: def_id :: { DefId , LocalDefId } ;}
mkuse!{use rustc_hir :: intravisit :: { self , Visitor , VisitorExt } ;}
mkuse!{use rustc_hir :: { self as hir , AmbigArg } ;}
mkuse!{use rustc_middle :: hir :: nested_filter ;}
mkuse!{use rustc_middle :: middle :: resolve_bound_vars as rbv ;}
mkuse!{use rustc_middle :: ty :: { self , Region , TyCtxt } ;}
mkuse!{use tracing :: debug ;}

macro_rules! find_anon_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_anon_type in module {}", module_path!());
    };
}

mkfn!{
    find_anon_type_introspect!();
    # [doc = " This function calls the `visit_ty` method for the parameters"] # [doc = " corresponding to the anonymous regions. The `nested_visitor.found_type`"] # [doc = " contains the anonymous type."] # [doc = ""] # [doc = " # Arguments"] # [doc = " region - the anonymous region corresponding to the anon_anon conflict"] # [doc = " br - the bound region corresponding to the above region which is of type `BrAnon(_)`"] # [doc = ""] # [doc = " # Example"] # [doc = " ```compile_fail"] # [doc = " fn foo(x: &mut Vec<&u8>, y: &u8)"] # [doc = "    { x.push(y); }"] # [doc = " ```"] # [doc = " The function returns the nested type corresponding to the anonymous region"] # [doc = " for e.g., `&u8` and `Vec<&u8>`."] pub fn find_anon_type < 'tcx > (tcx : TyCtxt < 'tcx > , generic_param_scope : LocalDefId , region : Region < 'tcx > ,) -> Option < (& 'tcx hir :: Ty < 'tcx > , & 'tcx hir :: FnSig < 'tcx >) > { let anon_reg = tcx . is_suitable_region (generic_param_scope , region) ? ; let fn_sig = tcx . hir_node_by_def_id (anon_reg . scope) . fn_sig () ? ; fn_sig . decl . inputs . iter () . find_map (| arg | find_component_for_bound_region (tcx , arg , anon_reg . region_def_id)) . map (| ty | (ty , fn_sig)) }
}

macro_rules! find_component_for_bound_region_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_component_for_bound_region in module {}", module_path!());
    };
}

mkfn!{
    find_component_for_bound_region_introspect!();
    fn find_component_for_bound_region < 'tcx > (tcx : TyCtxt < 'tcx > , arg : & 'tcx hir :: Ty < 'tcx > , region_def_id : DefId ,) -> Option < & 'tcx hir :: Ty < 'tcx > > { FindNestedTypeVisitor { tcx , region_def_id , current_index : ty :: INNERMOST } . visit_ty_unambig (arg) . break_value () }
}
mkitem!{mkstruct!{struct FindNestedTypeVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , region_def_id : DefId , current_index : ty :: DebruijnIndex , }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for FindNestedTypeVisitor < 'tcx > { type Result = ControlFlow < & 'tcx hir :: Ty < 'tcx > > ; type NestedFilter = nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_ty (& mut self , arg : & 'tcx hir :: Ty < 'tcx , AmbigArg >) -> Self :: Result { match arg . kind { hir :: TyKind :: FnPtr (_) => { self . current_index . shift_in (1) ; let _ = intravisit :: walk_ty (self , arg) ; self . current_index . shift_out (1) ; return ControlFlow :: Continue (()) ; } hir :: TyKind :: TraitObject (bounds , ..) => { for bound in bounds { self . current_index . shift_in (1) ; let _ = self . visit_poly_trait_ref (bound) ; self . current_index . shift_out (1) ; } } hir :: TyKind :: Ref (lifetime , _) => { let hir_id = lifetime . hir_id ; match self . tcx . named_bound_var (hir_id) { Some (rbv :: ResolvedArg :: EarlyBound (id)) => { debug ! ("EarlyBound id={:?}" , id) ; if id . to_def_id () == self . region_def_id { return ControlFlow :: Break (arg . as_unambig_ty ()) ; } } Some (rbv :: ResolvedArg :: LateBound (debruijn_index , _ , id)) => { debug ! ("FindNestedTypeVisitor::visit_ty: LateBound depth = {:?}" , debruijn_index) ; debug ! ("LateBound id={:?}" , id) ; if debruijn_index == self . current_index && id . to_def_id () == self . region_def_id { return ControlFlow :: Break (arg . as_unambig_ty ()) ; } } Some (rbv :: ResolvedArg :: StaticLifetime | rbv :: ResolvedArg :: Free (_ , _) | rbv :: ResolvedArg :: Error (_) ,) | None => { debug ! ("no arg found") ; } } } hir :: TyKind :: Path (_) => { intravisit :: walk_ty (self , arg) ? ; return if intravisit :: walk_ty (& mut TyPathVisitor { tcx : self . tcx , region_def_id : self . region_def_id , current_index : self . current_index , } , arg ,) . is_break () { ControlFlow :: Break (arg . as_unambig_ty ()) } else { ControlFlow :: Continue (()) } ; } _ => { } } intravisit :: walk_ty (self , arg) } }}}
mkitem!{mkstruct!{struct TyPathVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , region_def_id : DefId , current_index : ty :: DebruijnIndex , }}}
mkitem!{mkimpl!{impl < 'tcx > Visitor < 'tcx > for TyPathVisitor < 'tcx > { type Result = ControlFlow < () > ; type NestedFilter = nested_filter :: OnlyBodies ; fn maybe_tcx (& mut self) -> Self :: MaybeTyCtxt { self . tcx } fn visit_lifetime (& mut self , lifetime : & hir :: Lifetime) -> Self :: Result { match self . tcx . named_bound_var (lifetime . hir_id) { Some (rbv :: ResolvedArg :: EarlyBound (id)) => { debug ! ("EarlyBound id={:?}" , id) ; if id . to_def_id () == self . region_def_id { return ControlFlow :: Break (()) ; } } Some (rbv :: ResolvedArg :: LateBound (debruijn_index , _ , id)) => { debug ! ("FindNestedTypeVisitor::visit_ty: LateBound depth = {:?}" , debruijn_index ,) ; debug ! ("id={:?}" , id) ; if debruijn_index == self . current_index && id . to_def_id () == self . region_def_id { return ControlFlow :: Break (()) ; } } Some (rbv :: ResolvedArg :: StaticLifetime | rbv :: ResolvedArg :: Free (_ , _) | rbv :: ResolvedArg :: Error (_) ,) | None => { debug ! ("no arg found") ; } } ControlFlow :: Continue (()) } fn visit_ty (& mut self , arg : & 'tcx hir :: Ty < 'tcx , AmbigArg >) -> Self :: Result { debug ! ("`Ty` corresponding to a struct is {:?}" , arg) ; ControlFlow :: Continue (()) } }}}
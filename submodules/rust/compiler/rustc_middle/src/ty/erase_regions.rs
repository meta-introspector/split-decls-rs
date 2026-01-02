mkuse!{use tracing :: debug ;}
mkuse!{use crate :: query :: Providers ;}
mkuse!{use crate :: ty :: { self , Ty , TyCtxt , TypeFlags , TypeFoldable , TypeFolder , TypeSuperFoldable , TypeVisitableExt , } ;}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (super) fn provide (providers : & mut Providers) { * providers = Providers { erase_and_anonymize_regions_ty , .. * providers } ; }
}

macro_rules! erase_and_anonymize_regions_ty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function erase_and_anonymize_regions_ty in module {}", module_path!());
    };
}

mkfn!{
    erase_and_anonymize_regions_ty_introspect!();
    fn erase_and_anonymize_regions_ty < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx >) -> Ty < 'tcx > { ty . super_fold_with (& mut RegionEraserAndAnonymizerVisitor { tcx }) }
}
mkitem!{mkimpl!{impl < 'tcx > TyCtxt < 'tcx > { # [doc = " Returns an equivalent value with all free regions removed and"] # [doc = " bound regions anonymized. (note that bound regions are important"] # [doc = " for subtyping and generally type equality so *cannot* be removed)"] pub fn erase_and_anonymize_regions < T > (self , value : T) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { if ! value . has_type_flags (TypeFlags :: HAS_BINDER_VARS | TypeFlags :: HAS_FREE_REGIONS) { return value ; } debug ! ("erase_and_anonymize_regions({:?})" , value) ; let value1 = value . fold_with (& mut RegionEraserAndAnonymizerVisitor { tcx : self }) ; debug ! ("erase_and_anonymize_regions = {:?}" , value1) ; value1 } }}}
mkitem!{mkstruct!{struct RegionEraserAndAnonymizerVisitor < 'tcx > { tcx : TyCtxt < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for RegionEraserAndAnonymizerVisitor < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { if ! ty . has_type_flags (TypeFlags :: HAS_BINDER_VARS | TypeFlags :: HAS_FREE_REGIONS) { ty } else if ty . has_infer () { ty . super_fold_with (self) } else { self . tcx . erase_and_anonymize_regions_ty (ty) } } fn fold_binder < T > (& mut self , t : ty :: Binder < 'tcx , T >) -> ty :: Binder < 'tcx , T > where T : TypeFoldable < TyCtxt < 'tcx > > , { let u = self . tcx . anonymize_bound_vars (t) ; u . super_fold_with (self) } fn fold_region (& mut self , r : ty :: Region < 'tcx >) -> ty :: Region < 'tcx > { match r . kind () { ty :: ReBound (..) => r , _ => self . tcx . lifetimes . re_erased , } } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { if ct . has_type_flags (TypeFlags :: HAS_BINDER_VARS | TypeFlags :: HAS_FREE_REGIONS) { ct . super_fold_with (self) } else { ct } } fn fold_predicate (& mut self , p : ty :: Predicate < 'tcx >) -> ty :: Predicate < 'tcx > { if p . has_type_flags (TypeFlags :: HAS_BINDER_VARS | TypeFlags :: HAS_FREE_REGIONS) { p . super_fold_with (self) } else { p } } fn fold_clauses (& mut self , c : ty :: Clauses < 'tcx >) -> ty :: Clauses < 'tcx > { if c . has_type_flags (TypeFlags :: HAS_BINDER_VARS | TypeFlags :: HAS_FREE_REGIONS) { c . super_fold_with (self) } else { c } } }}}
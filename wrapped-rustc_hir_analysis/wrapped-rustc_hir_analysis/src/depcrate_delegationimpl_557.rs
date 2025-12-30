// Generated macro for impl_557 (impl)
macro_rules! Depcrate_delegationimpl_557 {
() => {
// Module: crate::delegation
// Provides: {"impl_557"}
// Dependencies: {}
impl < 'tcx > TypeFolder < TyCtxt < 'tcx > > for ParamIndexRemapper < 'tcx > { fn cx (& self) -> TyCtxt < 'tcx > { self . tcx } fn fold_ty (& mut self , ty : Ty < 'tcx >) -> Ty < 'tcx > { if ! ty . has_param () { return ty ; } if let ty :: Param (param) = ty . kind () && let Some (index) = self . remap_table . get (& param . index) { return Ty :: new_param (self . tcx , * index , param . name) ; } ty . super_fold_with (self) } fn fold_region (& mut self , r : ty :: Region < 'tcx >) -> ty :: Region < 'tcx > { if let ty :: ReEarlyParam (param) = r . kind () && let Some (index) = self . remap_table . get (& param . index) . copied () { return ty :: Region :: new_early_param (self . tcx , ty :: EarlyParamRegion { index , name : param . name } ,) ; } r } fn fold_const (& mut self , ct : ty :: Const < 'tcx >) -> ty :: Const < 'tcx > { if let ty :: ConstKind :: Param (param) = ct . kind () && let Some (idx) = self . remap_table . get (& param . index) { let param = ty :: ParamConst :: new (* idx , param . name) ; return ty :: Const :: new_param (self . tcx , param) ; } ct . super_fold_with (self) } }
};
}

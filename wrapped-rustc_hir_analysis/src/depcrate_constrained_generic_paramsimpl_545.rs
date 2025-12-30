// Generated macro for impl_545 (impl)
macro_rules! Depcrate_constrained_generic_paramsimpl_545 {
() => {
// Module: crate::constrained_generic_params
// Provides: {"impl_545"}
// Dependencies: {}
impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for ParameterCollector { fn visit_ty (& mut self , t : Ty < 'tcx >) { match * t . kind () { ty :: Alias (ty :: Projection | ty :: Inherent | ty :: Opaque , _) if ! self . include_nonconstraining => { return ; } ty :: Alias (ty :: Free , _) if ! self . include_nonconstraining => { bug ! ("unexpected free alias type") } ty :: Param (param) => self . parameters . push (Parameter :: from (param)) , _ => { } } t . super_visit_with (self) } fn visit_region (& mut self , r : ty :: Region < 'tcx >) { if let ty :: ReEarlyParam (data) = r . kind () { self . parameters . push (Parameter :: from (data)) ; } } fn visit_const (& mut self , c : ty :: Const < 'tcx >) { match c . kind () { ty :: ConstKind :: Unevaluated (..) if ! self . include_nonconstraining => { return ; } ty :: ConstKind :: Param (data) => { self . parameters . push (Parameter :: from (data)) ; } _ => { } } c . super_visit_with (self) } }
};
}

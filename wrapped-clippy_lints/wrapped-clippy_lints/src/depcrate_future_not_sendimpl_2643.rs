// Generated macro for impl_2643 (impl)
macro_rules! Depcrate_future_not_sendimpl_2643 {
() => {
// Module: crate::future_not_send
// Provides: {"impl_2643"}
// Dependencies: {}
impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for TyParamAtTopLevelVisitor { type Result = ControlFlow < bool > ; fn visit_ty (& mut self , ty : Ty < 'tcx >) -> Self :: Result { match ty . kind () { ty :: Param (_) => ControlFlow :: Break (true) , ty :: Alias (ty :: AliasTyKind :: Projection , ty) => ty . visit_with (self) , _ => ControlFlow :: Break (false) , } } }
};
}

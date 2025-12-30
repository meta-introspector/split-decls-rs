// Generated macro for get_enum_ty (function)
macro_rules! Depcrate_methods_useless_asrefget_enum_ty {
() => {
// Module: crate::methods::useless_asref
// Provides: {"get_enum_ty"}
// Dependencies: {}
# [doc = " Returns the first type inside the `Option`/`Result` type passed as argument."] fn get_enum_ty (enum_ty : Ty < '_ >) -> Option < Ty < '_ > > { struct ContainsTyVisitor { level : usize , } impl < 'tcx > TypeVisitor < TyCtxt < 'tcx > > for ContainsTyVisitor { type Result = ControlFlow < Ty < 'tcx > > ; fn visit_ty (& mut self , t : Ty < 'tcx >) -> Self :: Result { self . level += 1 ; if self . level == 1 { t . super_visit_with (self) } else { ControlFlow :: Break (t) } } } match enum_ty . visit_with (& mut ContainsTyVisitor { level : 0 }) { ControlFlow :: Break (ty) => Some (ty) , ControlFlow :: Continue (()) => None , } }
};
}

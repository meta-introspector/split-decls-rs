// Generated macro for VisitorExt (trait)
macro_rules! Depcrate_intravisitVisitorExt {
() => {
// Module: crate::intravisit
// Provides: {"VisitorExt"}
// Dependencies: {}
pub trait VisitorExt < 'v > : Visitor < 'v > { # [doc = " Extension trait method to visit types in unambiguous positions, this is not"] # [doc = " directly on the [`Visitor`] trait as this method should never be overridden."] # [doc = ""] # [doc = " Named `visit_ty_unambig` instead of `visit_unambig_ty` to aid in discovery"] # [doc = " by IDes when `v.visit_ty` is written."] fn visit_ty_unambig (& mut self , t : & 'v Ty < 'v >) -> Self :: Result { walk_unambig_ty (self , t) } # [doc = " Extension trait method to visit consts in unambiguous positions, this is not"] # [doc = " directly on the [`Visitor`] trait as this method should never be overridden."] # [doc = ""] # [doc = " Named `visit_const_arg_unambig` instead of `visit_unambig_const_arg` to aid in"] # [doc = " discovery by IDes when `v.visit_const_arg` is written."] fn visit_const_arg_unambig (& mut self , c : & 'v ConstArg < 'v >) -> Self :: Result { walk_unambig_const_arg (self , c) } }
};
}

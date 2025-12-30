// Generated macro for impl_43 (impl)
macro_rules! Depcrate_visit_dfsvisitimpl_43 {
() => {
// Module: crate::visit::dfsvisit
// Provides: {"impl_43"}
// Dependencies: {}
impl < B > Control < B > { pub fn breaking () -> Control < () > { Control :: Break (()) } # [doc = " Get the value in `Control::Break(_)`, if present."] pub fn break_value (self) -> Option < B > { match self { Control :: Continue | Control :: Prune => None , Control :: Break (b) => Some (b) , } } }
};
}

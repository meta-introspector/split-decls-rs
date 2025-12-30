// Generated macro for Certainty (enum)
macro_rules! Depcrate_ty_type_certainty_certaintyCertainty {
() => {
// Module: crate::ty::type_certainty::certainty
// Provides: {"Certainty"}
// Dependencies: {}
# [derive (Clone , Copy , Debug , PartialEq , Eq)] pub enum Certainty { # [doc = " Determining the type requires contextual information."] Uncertain , # [doc = " The type can be determined purely from subexpressions. If the argument is `Some(..)`, the"] # [doc = " specific `DefId` is known. Such arguments are needed to handle path segments whose `res` is"] # [doc = " `Res::Err`."] Certain (Option < DefId >) , # [doc = " The heuristic believes that more than one `DefId` applies to a type---this is a bug."] Contradiction , }
};
}

// Generated macro for Obligation (struct)
macro_rules! Depcrate_traitsObligation {
() => {
// Module: crate::traits
// Provides: {"Obligation"}
// Dependencies: {}
# [doc = " An `Obligation` represents some trait reference (e.g., `i32: Eq`) for"] # [doc = " which the \"impl_source\" must be found. The process of finding an \"impl_source\" is"] # [doc = " called \"resolving\" the `Obligation`. This process consists of"] # [doc = " either identifying an `impl` (e.g., `impl Eq for i32`) that"] # [doc = " satisfies the obligation, or else finding a bound that is in"] # [doc = " scope. The eventual result is usually a `Selection` (defined below)."] # [derive (Clone , TypeFoldable , TypeVisitable)] pub struct Obligation < 'tcx , T > { # [doc = " The reason we have to prove this thing."] # [type_foldable (identity)] # [type_visitable (ignore)] pub cause : ObligationCause < 'tcx > , # [doc = " The environment in which we should prove this thing."] pub param_env : ty :: ParamEnv < 'tcx > , # [doc = " The thing we are trying to prove."] pub predicate : T , # [doc = " If we started proving this as a result of trying to prove"] # [doc = " something else, track the total depth to ensure termination."] # [doc = " If this goes over a certain threshold, we abort compilation --"] # [doc = " in such cases, we can not say whether or not the predicate"] # [doc = " holds for certain. Stupid halting problem; such a drag."] # [type_foldable (identity)] # [type_visitable (ignore)] pub recursion_depth : usize , }
};
}

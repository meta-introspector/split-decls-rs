// Generated macro for MaybeOverride (trait)
macro_rules! Depcrate_precisionMaybeOverride {
() => {
// Module: crate::precision
// Provides: {"MaybeOverride"}
// Dependencies: {}
# [doc = " Allow overriding the outputs of specific test cases."] # [doc = ""] # [doc = " There are some cases where we want to xfail specific cases or handle certain inputs"] # [doc = " differently than the rest of calls to `validate`. This provides a hook to do that."] # [doc = ""] # [doc = " If `None` is returned, checks will proceed as usual. If `Some(result)` is returned, checks"] # [doc = " are skipped and the provided result is returned instead."] # [doc = ""] # [doc = " This gets implemented once per input type, then the functions provide further filtering"] # [doc = " based on function name and values."] # [doc = ""] # [doc = " `ulp` can also be set to adjust the ULP for that specific test, even if `None` is still"] # [doc = " returned."] pub trait MaybeOverride < Input > { fn check_float < F : Float > (_input : Input , _actual : F , _expected : F , _ctx : & CheckCtx ,) -> CheckAction { DEFAULT } fn check_int < I : Int > (_input : Input , _actual : I , _expected : I , _ctx : & CheckCtx) -> CheckAction { DEFAULT } }
};
}

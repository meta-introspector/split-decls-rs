// Generated macro for impl_87 (impl)
macro_rules! Depcrateimpl_87 {
() => {
// Module: crate
// Provides: {"impl_87"}
// Dependencies: {}
impl Function { # [doc = " Returns the `Function` value of this JS value if it's an instance of a"] # [doc = " function."] # [doc = ""] # [doc = " If this JS value is not an instance of a function then this returns"] # [doc = " `None`."] # [deprecated (note = "recommended to use dyn_ref instead which is now equivalent")] pub fn try_from (val : & JsValue) -> Option < & Function > { val . dyn_ref () } }
};
}

// Generated macro for impl_140 (impl)
macro_rules! Depcrateimpl_140 {
() => {
// Module: crate
// Provides: {"impl_140"}
// Dependencies: {}
impl Object { # [doc = " Returns the `Object` value of this JS value if it's an instance of an"] # [doc = " object."] # [doc = ""] # [doc = " If this JS value is not an instance of an object then this returns"] # [doc = " `None`."] pub fn try_from (val : & JsValue) -> Option < & Object > { if val . is_object () { Some (val . unchecked_ref ()) } else { None } } }
};
}

// Generated macro for prop_cast (function)
macro_rules! Depcrate_arg_msgargprop_cast {
() => {
// Module: crate::arg::msgarg
// Provides: {"prop_cast"}
// Dependencies: {}
# [doc = " Descend into a hashmap returned by e g \"Properties::get_all\" to retrieve the value of a property."] # [doc = ""] # [doc = " Shortcut for get + cast. Returns None both if the property does not exist, or if it was of a different type."] # [doc = " See the argument guide's reference section for which types you can cast to."] pub fn prop_cast < 'a , T : 'static > (map : & 'a PropMap , key : & str) -> Option < & 'a T > { map . get (key) . and_then (| v | cast (& v . 0)) }
};
}

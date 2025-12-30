// Generated macro for actor (macro)
macro_rules! Depcrate_utilactor {
() => {
// Module: crate::util
// Provides: {"actor"}
// Dependencies: {}
# [doc = " Creates an \"actor\" with default values, setters for all fields, and Clap parser support."] macro_rules ! actor { ($ (# [$ attr : meta]) + pub struct $ name : ident { $ ($ (# [$ field_attr : meta]) + $ field : ident : $ type : ty $ (= $ default : tt) *,) * }) => { $ (# [$ attr]) + # [derive (clap :: Args)] pub struct $ name { $ ($ (# [$ field_attr]) + # [arg (long , $ (default_value = $ default) *)] $ field : $ type ,) * } impl Default for $ name { fn default () -> $ name { $ name { $ ($ field : actor_field_default ! ($ (= $ default) *) ,) * } } } impl $ name { $ (pub fn $ field (& mut self , value : $ type) -> & mut Self { self .$ field = value ; self }) * } } }
};
}

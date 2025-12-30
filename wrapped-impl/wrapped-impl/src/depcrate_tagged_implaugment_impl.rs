// Generated macro for augment_impl (function)
macro_rules! Depcrate_tagged_implaugment_impl {
() => {
// Module: crate::tagged_impl
// Provides: {"augment_impl"}
// Dependencies: {}
fn augment_impl (input : & mut ItemImpl , name : & TokenStream , mode : Mode) { if mode . ser { input . items . push (parse_quote ! { # [doc (hidden)] fn typetag_name (& self) -> &'static str { # name } }) ; } if mode . de { input . items . push (parse_quote ! { # [doc (hidden)] fn typetag_deserialize (& self) { } }) ; } }
};
}

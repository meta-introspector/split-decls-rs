// Generated macro for augment_trait (function)
macro_rules! Depcrate_tagged_traitaugment_trait {
() => {
// Module: crate::tagged_trait
// Provides: {"augment_trait"}
// Dependencies: {}
fn augment_trait (input : & mut ItemTrait , mode : Mode) { if mode . ser { input . supertraits . push (parse_quote ! (typetag :: Serialize)) ; input . items . push (parse_quote ! { # [doc (hidden)] fn typetag_name (& self) -> &'static str ; }) ; } if mode . de { input . supertraits . push (parse_quote ! (typetag :: Deserialize)) ; input . items . push (parse_quote ! { # [doc (hidden)] fn typetag_deserialize (& self) ; }) ; } }
};
}

// Generated macro for has_a_default_variant (function)
macro_rules! Depcrate_deriving_defaulthas_a_default_variant {
() => {
// Module: crate::deriving::default
// Provides: {"has_a_default_variant"}
// Dependencies: {}
fn has_a_default_variant (item : & Annotatable) -> bool { struct HasDefaultAttrOnVariant ; impl < 'ast > rustc_ast :: visit :: Visitor < 'ast > for HasDefaultAttrOnVariant { type Result = ControlFlow < () > ; fn visit_variant (& mut self , v : & 'ast rustc_ast :: Variant) -> ControlFlow < () > { if v . attrs . iter () . any (| attr | attr . has_name (kw :: Default)) { ControlFlow :: Break (()) } else { ControlFlow :: Continue (()) } } } item . visit_with (& mut HasDefaultAttrOnVariant) . is_break () }
};
}

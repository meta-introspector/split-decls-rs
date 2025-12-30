// Generated macro for impl_80 (impl)
macro_rules! Depcrate_codegen_attrs_fieldimpl_80 {
() => {
// Module: crate::codegen::attrs_field
// Provides: {"impl_80"}
// Dependencies: {}
impl ForwardAttrs < '_ > { # [doc = " Check if this will forward any attributes; this requires both that"] # [doc = " there be a filter which can match some attributes and a field to receive them."] pub fn will_forward_any (& self) -> bool { if let Some (filter) = self . filter { ! filter . is_empty () && self . field . is_some () } else { false } } # [doc = " Get the field declarations to support attribute forwarding"] pub fn as_declaration (& self) -> Option < Declaration < '_ > > { self . field . map (Declaration) } # [doc = " Get the match arms for attribute matching"] pub fn as_match_arms (& self) -> MatchArms < '_ > { MatchArms (self) } # [doc = " Get the statement that will try to transform forwarded attributes into"] # [doc = " the result expected by the receiver field."] pub fn as_value_populator (& self) -> Option < ValuePopulator < '_ > > { self . field . map (ValuePopulator) } # [doc = " Get the field initializer for use when building the deriving struct."] pub fn as_initializer < 'a > (& 'a self) -> Option < impl 'a + ToTokens > { self . field . map (| f | f . as_initializer ()) } }
};
}

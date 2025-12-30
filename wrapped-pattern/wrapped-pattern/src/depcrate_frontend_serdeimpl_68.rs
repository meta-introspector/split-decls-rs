// Generated macro for impl_68 (impl)
macro_rules! Depcrate_frontend_serdeimpl_68 {
() => {
// Module: crate::frontend::serde
// Provides: {"impl_68"}
// Dependencies: {}
impl < B : PatternBackend > Serialize for Pattern < B > where B :: Store : Serialize , for < 'a > B :: PlaceholderKeyCow < 'a > : Serialize + From < B :: PlaceholderKey < 'a > > , { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if serializer . is_human_readable () { B :: iter_items (& self . store) . map (| x | x . into ()) . collect :: < HumanReadablePattern < B > > () . serialize (serializer) } else { self . store . serialize (serializer) } } }
};
}

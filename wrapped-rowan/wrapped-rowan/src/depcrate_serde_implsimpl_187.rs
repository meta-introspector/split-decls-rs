// Generated macro for impl_187 (impl)
macro_rules! Depcrate_serde_implsimpl_187 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_187"}
// Dependencies: {}
impl < L : Language > Serialize for SyntaxToken < L > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_map (Some (3)) ? ; state . serialize_entry ("kind" , & SerDisplay (DisplayDebug (self . kind ()))) ? ; state . serialize_entry ("text_range" , & self . text_range ()) ? ; state . serialize_entry ("text" , & self . text ()) ? ; state . end () } }
};
}

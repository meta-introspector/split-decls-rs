// Generated macro for impl_186 (impl)
macro_rules! Depcrate_serde_implsimpl_186 {
() => {
// Module: crate::serde_impls
// Provides: {"impl_186"}
// Dependencies: {}
impl < L : Language > Serialize for SyntaxNode < L > { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { let mut state = serializer . serialize_map (Some (3)) ? ; state . serialize_entry ("kind" , & SerDisplay (DisplayDebug (self . kind ()))) ? ; state . serialize_entry ("text_range" , & self . text_range ()) ? ; state . serialize_entry ("children" , & Children (self)) ? ; state . end () } }
};
}

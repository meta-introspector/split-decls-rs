// Generated macro for impl_37 (impl)
macro_rules! Depcrate_serdeimpl_37 {
() => {
// Module: crate::serde
// Provides: {"impl_37"}
// Dependencies: {}
impl Serialize for ByteSize { fn serialize < S > (& self , ser : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { if ser . is_human_readable () { < String > :: serialize (& self . to_string () , ser) } else { self . 0 . serialize (ser) } } }
};
}

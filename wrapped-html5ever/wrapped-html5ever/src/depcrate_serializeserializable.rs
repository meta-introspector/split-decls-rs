// Generated macro for Serializable (trait)
macro_rules! Depcrate_serializeSerializable {
() => {
// Module: crate::serialize
// Provides: {"Serializable"}
// Dependencies: {}
pub trait Serializable { fn serialize < 'wr , Wr : Write > (& self , serializer : & mut Serializer < 'wr , Wr > , traversal_scope : TraversalScope) -> io :: Result < () > ; }
};
}

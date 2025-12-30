// Generated macro for serialize_impl (macro)
macro_rules! Depcrate_serdeserialize_impl {
() => {
// Module: crate::serde
// Provides: {"serialize_impl"}
// Dependencies: {}
macro_rules ! serialize_impl { () => { fn serialize < Ser > (& self , serializer : Ser) -> Result < Ser :: Ok , Ser :: Error > where Ser : serde :: Serializer , { std :: ops :: Deref :: deref (self) . serialize (serializer) } } ; }
};
}

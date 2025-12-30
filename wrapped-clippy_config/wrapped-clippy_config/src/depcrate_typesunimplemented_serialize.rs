// Generated macro for unimplemented_serialize (macro)
macro_rules! Depcrate_typesunimplemented_serialize {
() => {
// Module: crate::types
// Provides: {"unimplemented_serialize"}
// Dependencies: {}
macro_rules ! unimplemented_serialize { ($ ($ t : ty ,) *) => { $ (impl Serialize for $ t { fn serialize < S > (& self , _serializer : S) -> Result < S :: Ok , S :: Error > where S : ser :: Serializer , { Err (ser :: Error :: custom ("unimplemented")) } }) * } }
};
}

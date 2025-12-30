// Generated macro for impl_range (macro)
macro_rules! Depcrate_deimpl_range {
() => {
// Module: crate::de
// Provides: {"impl_range"}
// Dependencies: {}
macro_rules ! impl_range { ($ type : ident , $ make : expr , $ ($ side : ident) ,*) => { impl < T : BorshDeserialize > BorshDeserialize for core :: ops ::$ type < T > { # [inline] fn deserialize_reader < R : Read > (reader : & mut R) -> Result < Self > { let ($ ($ side ,) *) = < _ >:: deserialize_reader (reader) ?; Ok ($ make) } } } ; }
};
}

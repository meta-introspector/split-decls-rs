// Generated macro for impl_69 (impl)
macro_rules! Depcrate_errorimpl_69 {
() => {
// Module: crate::error
// Provides: {"impl_69"}
// Dependencies: {}
impl Serialize for Error { fn serialize < S > (& self , serializer : S) -> core :: result :: Result < S :: Ok , S :: Error > where S : Serializer , { serializer . serialize_unit_variant ("Error" , self . clone () as u32 , VARIANT_NAMES [self . clone () as usize] ,) } }
};
}

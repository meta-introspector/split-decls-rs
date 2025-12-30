// Generated macro for impl_567 (impl)
macro_rules! Depcrate_ir_constantimpl_567 {
() => {
// Module: crate::ir::constant
// Provides: {"impl_567"}
// Dependencies: {}
impl fmt :: Display for ConstantData { # [doc = " Print the constant data in hexadecimal format, e.g. 0x000102030405060708090a0b0c0d0e0f."] # [doc = " This function will flip the stored order of bytes--little-endian--to the more readable"] # [doc = " big-endian ordering."] # [doc = ""] # [doc = " ```"] # [doc = " use cranelift_codegen::ir::ConstantData;"] # [doc = " let data = ConstantData::from([3, 2, 1, 0, 0].as_ref()); // note the little-endian order"] # [doc = " assert_eq!(data.to_string(), \"0x0000010203\");"] # [doc = " ```"] fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if ! self . is_empty () { write ! (f , "0x") ? ; for b in self . 0 . iter () . rev () { write ! (f , "{b:02x}") ? ; } } Ok (()) } }
};
}

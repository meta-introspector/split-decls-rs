// Generated macro for impl_bits (macro)
macro_rules! Depcrate_traitsimpl_bits {
() => {
// Module: crate::traits
// Provides: {"impl_bits"}
// Dependencies: {}
macro_rules ! impl_bits { ($ ($ u : ty , $ i : ty ,) *) => { $ (impl Bits for $ u { const EMPTY : $ u = 0 ; const ALL : $ u = <$ u >:: MAX ; } impl Bits for $ i { const EMPTY : $ i = 0 ; const ALL : $ i = <$ u >:: MAX as $ i ; } impl ParseHex for $ u { fn parse_hex (input : & str) -> Result < Self , ParseError > { <$ u >:: from_str_radix (input , 16) . map_err (| _ | ParseError :: invalid_hex_flag (input)) } } impl ParseHex for $ i { fn parse_hex (input : & str) -> Result < Self , ParseError > { <$ i >:: from_str_radix (input , 16) . map_err (| _ | ParseError :: invalid_hex_flag (input)) } } impl WriteHex for $ u { fn write_hex < W : fmt :: Write > (& self , mut writer : W) -> fmt :: Result { write ! (writer , "{:x}" , self) } } impl WriteHex for $ i { fn write_hex < W : fmt :: Write > (& self , mut writer : W) -> fmt :: Result { write ! (writer , "{:x}" , self) } } impl Primitive for $ i { } impl Primitive for $ u { }) * } }
};
}

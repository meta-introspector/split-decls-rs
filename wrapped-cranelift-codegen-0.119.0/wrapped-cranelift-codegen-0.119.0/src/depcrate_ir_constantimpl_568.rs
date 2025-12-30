// Generated macro for impl_568 (impl)
macro_rules! Depcrate_ir_constantimpl_568 {
() => {
// Module: crate::ir::constant
// Provides: {"impl_568"}
// Dependencies: {}
impl FromStr for ConstantData { type Err = & 'static str ; # [doc = " Parse a hexadecimal string to `ConstantData`. This is the inverse of `Display::fmt`."] # [doc = ""] # [doc = " ```"] # [doc = " use cranelift_codegen::ir::ConstantData;"] # [doc = " let c: ConstantData = \"0x000102\".parse().unwrap();"] # [doc = " assert_eq!(c.into_vec(), [2, 1, 0]);"] # [doc = " ```"] fn from_str (s : & str) -> Result < Self , & 'static str > { if s . len () <= 2 || & s [0 .. 2] != "0x" { return Err ("Expected a hexadecimal string, e.g. 0x1234") ; } let cleaned : Vec < u8 > = s [2 ..] . as_bytes () . iter () . filter (| & & b | b as char != '_') . cloned () . collect () ; if cleaned . is_empty () { Err ("Hexadecimal string must have some digits") } else if cleaned . len () % 2 != 0 { Err ("Hexadecimal string must have an even number of digits") } else if cleaned . len () > 32 { Err ("Hexadecimal string has too many digits to fit in a 128-bit vector") } else { let mut buffer = Vec :: with_capacity ((s . len () - 2) / 2) ; for i in (0 .. cleaned . len ()) . step_by (2) { let pair = from_utf8 (& cleaned [i .. i + 2]) . or_else (| _ | Err ("Unable to parse hexadecimal pair as UTF-8")) ? ; let byte = u8 :: from_str_radix (pair , 16) . or_else (| _ | Err ("Unable to parse as hexadecimal")) ? ; buffer . insert (0 , byte) ; } Ok (Self (buffer)) } } }
};
}

// Generated macro for impl_129 (impl)
macro_rules! Depcrateimpl_129 {
() => {
// Module: crate
// Provides: {"impl_129"}
// Dependencies: {}
impl < T : Num + Clone > Num for Complex < T > { type FromStrRadixErr = ParseComplexError < T :: FromStrRadixErr > ; # [doc = " Parses `a +/- bi`; `ai +/- b`; `a`; or `bi` where `a` and `b` are of type `T`"] # [doc = ""] # [doc = " `radix` must be <= 18; larger radix would include *i* and *j* as digits,"] # [doc = " which cannot be supported."] # [doc = ""] # [doc = " The conversion returns an error if 18 <= radix <= 36; it panics if radix > 36."] # [doc = ""] # [doc = " The elements of `T` are parsed using `Num::from_str_radix` too, and errors"] # [doc = " (or panics) from that are reflected here as well."] fn from_str_radix (s : & str , radix : u32) -> Result < Self , Self :: FromStrRadixErr > { assert ! (radix <= 36 , "from_str_radix: radix is too high (maximum 36)") ; if radix > 18 { return Err (ParseComplexError :: unsupported_radix ()) ; } from_str_generic (s , | x | -> Result < T , T :: FromStrRadixErr > { T :: from_str_radix (x , radix) }) } }
};
}

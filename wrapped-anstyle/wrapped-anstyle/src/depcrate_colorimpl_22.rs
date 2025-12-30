// Generated macro for impl_22 (impl)
macro_rules! Depcrate_colorimpl_22 {
() => {
// Module: crate::color
// Provides: {"impl_22"}
// Dependencies: {}
impl DisplayBuffer { # [must_use] # [inline (never)] fn write_str (mut self , part : & 'static str) -> Self { for (i , b) in part . as_bytes () . iter () . enumerate () { self . buffer [self . len + i] = * b ; } self . len += part . len () ; self } # [must_use] # [inline (never)] fn write_code (mut self , code : u8) -> Self { let c1 : u8 = (code / 100) % 10 ; let c2 : u8 = (code / 10) % 10 ; let c3 : u8 = code % 10 ; let mut printed = false ; if c1 != 0 { printed = true ; self . buffer [self . len] = b'0' + c1 ; self . len += 1 ; } if c2 != 0 || printed { self . buffer [self . len] = b'0' + c2 ; self . len += 1 ; } self . buffer [self . len] = b'0' + c3 ; self . len += 1 ; self } # [inline] fn as_str (& self) -> & str { # [allow (unsafe_code)] unsafe { core :: str :: from_utf8_unchecked (& self . buffer [0 .. self . len]) } } # [inline] # [cfg (feature = "std")] fn write_to (self , write : & mut dyn std :: io :: Write) -> std :: io :: Result < () > { write . write_all (self . as_str () . as_bytes ()) } }
};
}

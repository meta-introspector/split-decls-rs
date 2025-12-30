// Generated macro for proptest_from_utf16 (function)
macro_rules! Depcrate_testsproptest_from_utf16 {
() => {
// Module: crate::tests
// Provides: {"proptest_from_utf16"}
// Dependencies: {}
# [proptest] # [cfg_attr (miri , ignore)] fn proptest_from_utf16 (# [strategy (rand_u16s ())] buf : Vec < u16 >) { type FromUtf16Func = fn (& [u8]) -> Result < CompactString , crate :: Utf16Error > ; type FromU16Endian = fn (u16) -> u16 ; type FromU16EndianBytes = fn ([u8 ; 2]) -> u16 ; const FUNCS : & [(FromUtf16Func , FromU16Endian , FromU16EndianBytes)] = & [(| v | CompactString :: from_utf16le (v) , u16 :: from_le , u16 :: from_le_bytes ,) , (| v | CompactString :: from_utf16be (v) , u16 :: from_be , u16 :: from_be_bytes ,) ,] ; for (new_compact_string , from_int , from_bytes) in FUNCS { let buf = & * buf ; let bytes : & [u8] = unsafe { slice :: from_raw_parts (buf . as_ptr () . cast () , buf . len () * 2) } ; let compact = new_compact_string (bytes) ; let control = String :: from_utf16 (& buf . iter () . copied () . map (from_int) . collect :: < Vec < u16 > > ()) ; assert_eq ! (compact . is_ok () , control . is_ok ()) ; if let (Ok (compact) , Ok (control)) = (compact , control) { assert_eq ! (compact . len () , control . len ()) ; assert_eq ! (compact , control) ; } if bytes . len () >= 2 { let bytes : & [u8] = & bytes [1 .. bytes . len () - 1] ; let buf : Vec < u16 > = bytes . chunks_exact (2) . map (| v | from_bytes ([v [0] , v [1]])) . collect () ; let compact = new_compact_string (bytes) ; let control = String :: from_utf16 (& buf) ; assert_eq ! (compact . is_ok () , control . is_ok ()) ; if let (Ok (compact) , Ok (control)) = (compact , control) { assert_eq ! (compact . len () , control . len ()) ; assert_eq ! (compact , control) ; } } } }
};
}

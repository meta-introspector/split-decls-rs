// Generated macro for tests (module)
macro_rules! Depcrate_commontests {
() => {
// Module: crate::common
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use asn1 :: Asn1Readable ; use super :: { Asn1ReadableOrWritable , RawTlv , UnvalidatedVisibleString , WithTlv } ; # [test] # [should_panic] fn test_unvalidated_visible_string_write () { let v = UnvalidatedVisibleString ("foo") ; asn1 :: write_single (& v) . unwrap () ; } # [test] # [should_panic] fn test_asn1_readable_or_writable_unwrap_read () { Asn1ReadableOrWritable :: < u32 , u32 > :: new_write (17) . unwrap_read () ; } # [test] fn test_asn1_readable_or_writable_write_read_data () { let v = Asn1ReadableOrWritable :: < u32 , u32 > :: new_read (17) ; assert_eq ! (& asn1 :: write_single (& v) . unwrap () , b"\x02\x01\x11") ; } # [test] fn test_raw_tlv_can_parse () { let t = asn1 :: Tag :: from_bytes (& [0]) . unwrap () . 0 ; assert ! (RawTlv :: can_parse (t)) ; } # [test] fn test_with_raw_tlv_can_parse () { let t = asn1 :: Tag :: from_bytes (& [0x30]) . unwrap () . 0 ; assert ! (WithTlv ::< asn1 :: Sequence <'_ >>:: can_parse (t)) ; assert ! (! WithTlv ::< bool >:: can_parse (t)) ; } }
};
}

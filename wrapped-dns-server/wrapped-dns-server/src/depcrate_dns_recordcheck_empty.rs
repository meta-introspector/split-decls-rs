// Generated macro for check_empty (function)
macro_rules! Depcrate_dns_recordcheck_empty {
() => {
// Module: crate::dns_record
// Provides: {"check_empty"}
// Dependencies: {}
fn check_empty < const SIZE : usize > (f : & FixedBuf < SIZE >) -> Result < () , DnsError > { if f . is_empty () { Ok (()) } else { Err (DnsError :: RecordHasAdditionalBytes) } }
};
}

// Generated macro for impl_43 (impl)
macro_rules! Depcrate_dns_questionimpl_43 {
() => {
// Module: crate::dns_question
// Provides: {"impl_43"}
// Dependencies: {}
impl DnsQuestion { # [doc = " # Errors"] # [doc = " Returns an error when `buf` does not contain a valid question struct."] pub fn read < const N : usize > (buf : & mut FixedBuf < N >) -> Result < Self , DnsError > { let name = DnsName :: read (buf) ? ; let typ = DnsType :: read (buf) ? ; let class = DnsClass :: read (buf) ? ; if class != DnsClass :: Internet && class != DnsClass :: Any { return Err (DnsError :: InvalidClass) ; } Ok (DnsQuestion { name , typ , class }) } # [doc = " # Errors"] # [doc = " Returns an error when `buf` fills up."] pub fn write < const N : usize > (& self , out : & mut FixedBuf < N >) -> Result < () , DnsError > { self . name . write (out) ? ; self . typ . write (out) ? ; self . class . write (out) ? ; Ok (()) } }
};
}

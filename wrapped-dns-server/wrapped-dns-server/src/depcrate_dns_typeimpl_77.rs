// Generated macro for impl_77 (impl)
macro_rules! Depcrate_dns_typeimpl_77 {
() => {
// Module: crate::dns_type
// Provides: {"impl_77"}
// Dependencies: {}
impl DnsType { # [must_use] pub fn new (value : u16) -> Self { match value { 1 => DnsType :: A , 28 => DnsType :: AAAA , 5 => DnsType :: CNAME , 15 => DnsType :: MX , 2 => DnsType :: NS , 12 => DnsType :: PTR , 6 => DnsType :: SOA , 16 => DnsType :: TXT , 255 => DnsType :: ANY , other => DnsType :: Unknown (other) , } } # [must_use] pub fn num (& self) -> u16 { match self { DnsType :: A => 1 , DnsType :: AAAA => 28 , DnsType :: CNAME => 5 , DnsType :: MX => 15 , DnsType :: NS => 2 , DnsType :: PTR => 12 , DnsType :: SOA => 6 , DnsType :: TXT => 16 , DnsType :: ANY => 255 , DnsType :: Unknown (other) => * other , } } # [doc = " # Errors"] # [doc = " Returns an error when `buf` does not contain a valid two-byte type code."] pub fn read < const N : usize > (buf : & mut FixedBuf < N >) -> Result < Self , DnsError > { Ok (Self :: new (read_u16_be (buf) ?)) } # [doc = " # Errors"] # [doc = " Returns an error when `buf` fills up."] pub fn write < const N : usize > (& self , out : & mut FixedBuf < N >) -> Result < () , DnsError > { write_u16_be (out , self . num ()) } }
};
}

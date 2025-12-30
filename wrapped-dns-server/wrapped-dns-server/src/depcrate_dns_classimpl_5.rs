// Generated macro for impl_5 (impl)
macro_rules! Depcrate_dns_classimpl_5 {
() => {
// Module: crate::dns_class
// Provides: {"impl_5"}
// Dependencies: {}
impl DnsClass { # [must_use] pub fn new (value : u16) -> Self { match value { 1 => DnsClass :: Internet , 255 => DnsClass :: Any , other => DnsClass :: Unknown (other) , } } # [must_use] pub fn num (& self) -> u16 { match self { DnsClass :: Internet => 1 , DnsClass :: Any => 255 , DnsClass :: Unknown (other) => * other , } } # [doc = " # Errors"] # [doc = " Returns an error when `buf` does not contain two bytes."] pub fn read < const N : usize > (buf : & mut FixedBuf < N >) -> Result < Self , DnsError > { Ok (Self :: new (read_u16_be (buf) ?)) } # [doc = " # Errors"] # [doc = " Returns an error when `buf` is full."] pub fn write < const N : usize > (& self , out : & mut FixedBuf < N >) -> Result < () , DnsError > { write_u16_be (out , self . num ()) } }
};
}

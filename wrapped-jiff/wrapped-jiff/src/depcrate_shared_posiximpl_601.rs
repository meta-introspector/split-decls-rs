// Generated macro for impl_601 (impl)
macro_rules! Depcrate_shared_posiximpl_601 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_601"}
// Dependencies: {}
impl PosixTimeZone < Abbreviation > { # [doc = " Parse a POSIX `TZ` environment variable, assuming it's a rule and not"] # [doc = " an implementation defined value, from the given bytes."] # [cfg (feature = "alloc")] pub fn parse (bytes : & [u8]) -> Result < PosixTimeZone < Abbreviation > , Error > { let parser = Parser { ianav3plus : true , .. Parser :: new (bytes) } ; parser . parse () } # [doc = " Like parse, but parses a prefix of the input given and returns whatever"] # [doc = " is remaining."] # [cfg (feature = "alloc")] pub fn parse_prefix < 'b > (bytes : & 'b [u8] ,) -> Result < (PosixTimeZone < Abbreviation > , & 'b [u8]) , Error > { let parser = Parser { ianav3plus : true , .. Parser :: new (bytes) } ; parser . parse_prefix () } }
};
}

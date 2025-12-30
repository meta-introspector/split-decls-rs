// Generated macro for impl_35 (impl)
macro_rules! Depcrate_shared_posiximpl_35 {
() => {
// Module: crate::shared::posix
// Provides: {"impl_35"}
// Dependencies: {}
impl PosixTimeZone < Abbreviation > { # [doc = " Parse a POSIX `TZ` environment variable, assuming it's a rule and not"] # [doc = " an implementation defined value, from the given bytes."] pub fn parse (bytes : & [u8]) -> Result < PosixTimeZone < Abbreviation > , Error > { let parser = Parser { ianav3plus : true , .. Parser :: new (bytes) } ; parser . parse () } }
};
}

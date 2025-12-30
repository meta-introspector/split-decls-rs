// Generated macro for hash (function)
macro_rules! Depcrate_checksumhash {
() => {
// Module: crate::checksum
// Provides: {"hash"}
// Dependencies: {}
fn hash (path : & Path) -> Result < String , Box < dyn Error > > { let mut file = BufReader :: new (File :: open (path) ?) ; let mut sha256 = Sha256 :: default () ; std :: io :: copy (& mut file , & mut sha256) ? ; Ok (hex :: encode (sha256 . finalize ())) }
};
}

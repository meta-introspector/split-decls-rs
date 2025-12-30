// Generated macro for impl_30 (impl)
macro_rules! Depcrateimpl_30 {
() => {
// Module: crate
// Provides: {"impl_30"}
// Dependencies: {}
impl FromStr for CipherSuite { type Err = anyhow :: Error ; fn from_str (s : & str) -> Result < Self , Self :: Err > { match s . to_lowercase () . as_str () { "aes128" => Ok (CipherSuite :: Aes128) , "aes256" => Ok (CipherSuite :: Aes256) , "chacha20" => Ok (CipherSuite :: Chacha20) , _ => Err (anyhow :: anyhow ! ("Unknown cipher suite {}" , s)) , } } }
};
}

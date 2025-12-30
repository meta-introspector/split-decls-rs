// Generated macro for impl_207 (impl)
macro_rules! Depcrate_certimpl_207 {
() => {
// Module: crate::cert
// Provides: {"impl_207"}
// Dependencies: {}
impl SshHostKeyType { # [doc = " The name of the key type as encoded in the known_hosts file."] pub fn name (& self) -> & 'static str { match self { SshHostKeyType :: Unknown => "unknown" , SshHostKeyType :: Rsa => "ssh-rsa" , SshHostKeyType :: Dss => "ssh-dss" , SshHostKeyType :: Ecdsa256 => "ecdsa-sha2-nistp256" , SshHostKeyType :: Ecdsa384 => "ecdsa-sha2-nistp384" , SshHostKeyType :: Ecdsa521 => "ecdsa-sha2-nistp521" , SshHostKeyType :: Ed255219 => "ssh-ed25519" , } } # [doc = " A short name of the key type, the colloquial form used as a human-readable description."] pub fn short_name (& self) -> & 'static str { match self { SshHostKeyType :: Unknown => "Unknown" , SshHostKeyType :: Rsa => "RSA" , SshHostKeyType :: Dss => "DSA" , SshHostKeyType :: Ecdsa256 => "ECDSA" , SshHostKeyType :: Ecdsa384 => "ECDSA" , SshHostKeyType :: Ecdsa521 => "ECDSA" , SshHostKeyType :: Ed255219 => "ED25519" , } } }
};
}

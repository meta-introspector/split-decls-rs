// Generated macro for impl_119 (impl)
macro_rules! Depcrateimpl_119 {
() => {
// Module: crate
// Provides: {"impl_119"}
// Dependencies: {}
impl fmt :: Display for PasswordHash < '_ > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}{}" , PASSWORD_HASH_SEPARATOR , self . algorithm) ? ; if let Some (version) = self . version { write ! (f , "{PASSWORD_HASH_SEPARATOR}v={version}") ? ; } if ! self . params . is_empty () { write ! (f , "{}{}" , PASSWORD_HASH_SEPARATOR , self . params) ? ; } if let Some (salt) = & self . salt { write ! (f , "{PASSWORD_HASH_SEPARATOR}{salt}") ? ; if let Some (hash) = & self . hash { write ! (f , "{PASSWORD_HASH_SEPARATOR}{hash}") ? ; } } Ok (()) } }
};
}

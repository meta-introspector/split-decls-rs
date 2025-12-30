// Generated macro for impl_662 (impl)
macro_rules! Depcrate_configimpl_662 {
() => {
// Module: crate::config
// Provides: {"impl_662"}
// Dependencies: {}
impl ValidationTokenConfig { # [doc = " Duration after an address validation token was issued for which it's considered valid"] # [doc = ""] # [doc = " This refers only to tokens sent in NEW_TOKEN frames, in contrast to retry tokens."] # [doc = ""] # [doc = " Defaults to 2 weeks."] pub fn lifetime (& mut self , value : Duration) -> & mut Self { self . lifetime = value ; self } # [allow (rustdoc :: redundant_explicit_links)] # [doc = " Set a custom [`TokenLog`]"] # [doc = ""] # [doc = " If the `bloom` feature is enabled (which it is by default), defaults to a default"] # [doc = " [`BloomTokenLog`][crate::BloomTokenLog], which is suitable for most internet applications."] # [doc = ""] # [doc = " If the `bloom` feature is disabled, defaults to [`NoneTokenLog`][crate::NoneTokenLog],"] # [doc = " which makes the server ignore all address validation tokens (that is, tokens originating"] # [doc = " from NEW_TOKEN frames--retry tokens are not affected)."] pub fn log (& mut self , log : Arc < dyn TokenLog >) -> & mut Self { self . log = log ; self } # [doc = " Number of address validation tokens sent to a client when its path is validated"] # [doc = ""] # [doc = " This refers only to tokens sent in NEW_TOKEN frames, in contrast to retry tokens."] # [doc = ""] # [doc = " If the `bloom` feature is enabled (which it is by default), defaults to 2. Otherwise,"] # [doc = " defaults to 0."] pub fn sent (& mut self , value : u32) -> & mut Self { self . sent = value ; self } }
};
}

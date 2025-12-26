use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl PasswordClient {
    /// Builds a new `PasswordClient`.
    ///
    /// See example at [`PasswordClientBuilder`].
    pub fn builder() -> PasswordClientBuilder {
        PasswordClientBuilder::default()
    }
    /// Responds to the challenge with the supplied parameters.
    ///
    /// The caller should use the returned string as an `Authorization` or
    /// `Proxy-Authorization` header value.
    #[allow(unused_variables)]
    pub fn respond(&mut self, p: &PasswordParams) -> Result<String, String> {
        match self {
            #[cfg(feature = "basic-scheme")]
            Self::Basic(c) => Ok(c.respond(p.username, p.password)),
            #[cfg(feature = "digest-scheme")]
            Self::Digest(c) => c.respond(p),
            #[cfg(not(any(feature = "basic-scheme", feature = "digest-scheme")))]
            _ => unreachable!(),
        }
    }
}

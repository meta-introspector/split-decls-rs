use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Client for responding to a password challenge.
///
/// Typically created via [`TryFrom`] implementations for a parsed challenge
/// ([`crate::ChallengeRef`]) or unparsed challenges (`str`,
/// [`http::header::HeaderValue`], or [`http::header::GetAll`]). See full
/// example in the [crate-level documentation](crate).
///
/// For more complex scenarios, see [`PasswordClientBuilder`].
#[derive(Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum PasswordClient {
    #[cfg(feature = "basic-scheme")]
    #[cfg_attr(docsrs, doc(cfg(feature = "basic-scheme")))]
    Basic(BasicClient),
    #[cfg(feature = "digest-scheme")]
    #[cfg_attr(docsrs, doc(cfg(feature = "digest-scheme")))]
    Digest(DigestClient),
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Builds a [`PasswordClient`] from the supplied challenges; create via
/// [`PasswordClient::builder`].
///
/// Often you can just use [`PasswordClient`]'s [`TryFrom`] implementations
/// to convert from a parsed challenge ([`crate::ChallengeRef`]) or
/// unparsed challenges (`str`, [`http::header::HeaderValue`], or
/// [`http::header::GetAll`]).
///
/// The builder allows more flexibility. For example, if you are using a HTTP
/// library which is not based on a `http` crate, you might need to create
/// a `PasswordClient` from an iterator over multiple `WWW-Authenticate`
/// headers. You can feed each to [`PasswordClientBuilder::challenges`].
///
/// Prefers `Digest` over `Basic`, consistent with the [RFC 7235 section
/// 2.1](https://datatracker.ietf.org/doc/html/rfc7235#section-2.1) advice
/// for a user-agent to pick the most secure auth-scheme it understands.
///
/// When there are multiple `Digest` challenges, currently uses the first,
/// consistent with the [RFC 7616 section
/// 3.7](https://datatracker.ietf.org/doc/html/rfc7616#section-3.7)
/// advice to "use the first challenge it supports, unless a local policy
/// dictates otherwise". In the future, it may prioritize by algorithm.
///
/// Ignores parse errors as long as there's at least one parseable, supported
/// challenge.
///
/// ## Example
///
#[cfg_attr(
    feature = "digest",
    doc = r##"
```rust
use http_auth::PasswordClient;
let client = PasswordClient::builder()
    .challenges("UnsupportedSchemeA, Basic realm=\"foo\", UnsupportedSchemeB")
    .challenges("Digest \
                 realm=\"http-auth@example.org\", \
                 qop=\"auth, auth-int\", \
                 algorithm=MD5, \
                 nonce=\"7ypf/xlj9XXwfDPEoM4URrv/xwf94BcCAzFZH4GiTo0v\", \
                 opaque=\"FQhe/qaU925kfnzjCev0ciny7QMkPqMAFRtzCUYo5tdS\"")
    .build()
    .unwrap();
assert!(matches!(client, PasswordClient::Digest(_)));
```
"##
)]
#[derive(Default)]
pub struct PasswordClientBuilder(
    /// The current result:
    /// *   `Some(Ok(_))` if there is a suitable client.
    /// *   `Some(Err(_))` if there is no suitable client and has been a parse error.
    /// *   `None` otherwise.
    Option<Result<PasswordClient, String>>,
);

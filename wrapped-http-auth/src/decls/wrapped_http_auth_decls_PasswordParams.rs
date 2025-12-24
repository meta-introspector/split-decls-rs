use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Parameters for responding to a password challenge.
///
/// This is cheap to construct; callers generally use a fresh `PasswordParams`
/// for each request.
///
/// The caller is responsible for supplying parameters in the correct
/// format. Servers may expect character data to be in Unicode Normalization
/// Form C as noted in [RFC 7617 section
/// 2.1](https://datatracker.ietf.org/doc/html/rfc7617#section-2.1) for the
/// `Basic` scheme and [RFC 7616 section
/// 4](https://datatracker.ietf.org/doc/html/rfc7616#section-4) for the `Digest`
/// scheme.
///
/// Note that most of these fields are only needed for [`DigestClient`]. Callers
/// that only care about the `Basic` challenge scheme can use
/// [`BasicClient::respond`] directly with only username and password.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PasswordParams<'a> {
    pub username: &'a str,
    pub password: &'a str,
    /// The URI from the Request-URI of the Request-Line, as described in
    /// [RFC 2617 section 3.2.2](https://datatracker.ietf.org/doc/html/rfc2617#section-3.2.2).
    ///
    /// [RFC 2617 section
    /// 3.2.2.5](https://datatracker.ietf.org/doc/html/rfc2617#section-3.2.2.5),
    /// which says the following:
    /// > This may be `*`, an `absoluteURL` or an `abs_path` as specified in
    /// > section 5.1.2 of [RFC 2616](https://datatracker.ietf.org/doc/html/rfc2616),
    /// > but it MUST agree with the Request-URI. In particular, it MUST
    /// > be an `absoluteURL` if the Request-URI is an `absoluteURL`.
    ///
    /// [RFC 7616 section 3.4](https://datatracker.ietf.org/doc/html/rfc7616#section-3.4)
    /// describes this as the "Effective Request URI", which is *always* an
    /// absolute form. This may be a mistake. [Section
    /// 3.4.6](https://datatracker.ietf.org/doc/html/rfc7616#section-3.4.6)
    /// matches RFC 2617 section 3.2.2.5, and [Appendix
    /// A](https://datatracker.ietf.org/doc/html/rfc7616#appendix-A) doesn't
    /// mention a change from RFC 2617.
    pub uri: &'a str,
    /// The HTTP method, such as `GET`.
    ///
    /// When using the `http` crate, use the return value of
    /// [`http::Method::as_str`].
    pub method: &'a str,
    /// The entity body, if available. Use `Some(&[])` for HTTP methods with no
    /// body.
    ///
    /// When `None`, `Digest` challenges will only be able to use
    /// [`crate::digest::Qop::Auth`], not
    /// [`crate::digest::Qop::AuthInt`].
    pub body: Option<&'a [u8]>,
}

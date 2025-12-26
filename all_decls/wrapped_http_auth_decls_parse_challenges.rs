use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Parses a list of challenges into a `Vec`.
///
/// Most callers don't need to directly parse; see [`PasswordClient`] instead.
///
/// This is a shorthand for `parser::ChallengeParser::new(input).collect()`. Use
/// [`crate::parser::ChallengeParser`] directly when you want to parse lazily,
/// avoid allocation, and/or see any well-formed challenges before an error.
///
/// ## Example
///
/// ```rust
/// use http_auth::{parse_challenges, ChallengeRef, ParamValue};
///
/// // When all challenges are well-formed, returns them.
/// assert_eq!(
///     parse_challenges("UnsupportedSchemeA, Basic realm=\"foo\"").unwrap(),
///     vec![
///         ChallengeRef {
///             scheme: "UnsupportedSchemeA",
///             params: vec![],
///         },
///         ChallengeRef {
///             scheme: "Basic",
///             params: vec![("realm", ParamValue::try_from_escaped("foo").unwrap())],
///         },
///     ],
/// );
///
/// // Returns `Err` if there is a syntax error anywhere in the input.
/// parse_challenges("UnsupportedSchemeA, Basic realm=\"foo\", error error").unwrap_err();
/// ```
#[inline]
pub fn parse_challenges(input: &str) -> Result<Vec<ChallengeRef<'_>>, parser::Error<'_>> {
    parser::ChallengeParser::new(input).collect()
}

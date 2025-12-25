use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl PasswordClientBuilder {
    /// Considers all challenges from the given [`http::HeaderValue`] challenge list.
    #[cfg(any(feature = "http", feature = "http10"))]
    #[cfg_attr(docsrs, doc(cfg(any(feature = "http", feature = "http10"))))]
    pub fn header_value<V: HeaderValue>(mut self, value: &V) -> Self {
        if self.complete() {
            return self;
        }
        match value.to_str() {
            Ok(v) => self = self.challenges(v),
            Err(_) if matches!(self.0, None) => {
                self.0 = Some(Err("non-ASCII header value".into()));
            }
            _ => {}
        }
        self
    }
    /// Returns true if no more challenges need to be examined.
    #[cfg(feature = "digest-scheme")]
    fn complete(&self) -> bool {
        matches!(self.0, Some(Ok(PasswordClient::Digest(_))))
    }
    /// Returns true if no more challenges need to be examined.
    #[cfg(not(feature = "digest-scheme"))]
    fn complete(&self) -> bool {
        matches!(self.0, Some(Ok(_)))
    }
    /// Considers all challenges from the given `&str` challenge list.
    pub fn challenges(mut self, value: &str) -> Self {
        let mut parser = ChallengeParser::new(value);
        while !self.complete() {
            match parser.next() {
                Some(Ok(c)) => self = self.challenge(&c),
                Some(Err(e)) if self.0.is_none() => self.0 = Some(Err(e.to_string())),
                _ => break,
            }
        }
        self
    }
    /// Considers a single challenge.
    pub fn challenge(mut self, challenge: &ChallengeRef<'_>) -> Self {
        if self.complete() {
            return self;
        }
        #[cfg(feature = "digest-scheme")]
        if challenge.scheme.eq_ignore_ascii_case("Digest") {
            match DigestClient::try_from(challenge) {
                Ok(c) => self.0 = Some(Ok(PasswordClient::Digest(c))),
                Err(e) if self.0.is_none() => self.0 = Some(Err(e)),
                _ => {}
            }
            return self;
        }
        #[cfg(feature = "basic-scheme")]
        if challenge.scheme.eq_ignore_ascii_case("Basic") && !matches!(self.0, Some(Ok(_))) {
            match BasicClient::try_from(challenge) {
                Ok(c) => self.0 = Some(Ok(PasswordClient::Basic(c))),
                Err(e) if self.0.is_none() => self.0 = Some(Err(e)),
                _ => {}
            }
            return self;
        }
        if self.0.is_none() {
            self.0 = Some(Err(format!("Unsupported scheme {:?}", challenge.scheme)));
        }
        self
    }
    /// Returns a new [`PasswordClient`] or fails.
    pub fn build(self) -> Result<PasswordClient, String> {
        self.0.unwrap_or_else(|| Err("no challenges given".into()))
    }
}

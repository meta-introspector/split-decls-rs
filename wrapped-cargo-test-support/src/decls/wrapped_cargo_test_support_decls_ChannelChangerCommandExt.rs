use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Enable nightly features for testing
pub trait ChannelChangerCommandExt {
    /// The list of reasons should be why nightly cargo is needed. If it is
    /// because of an unstable feature put the name of the feature as the reason,
    /// e.g. `&["print-im-a-teapot"]`.
    fn masquerade_as_nightly_cargo(self, _reasons: &[&str]) -> Self;
}

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Debug)]
pub struct SsrRule {
    /// A structured pattern that we're searching for.
    pattern: parsing::RawPattern,
    /// What we'll replace it with.
    template: parsing::RawPattern,
    parsed_rules: Vec<parsing::ParsedRule>,
}

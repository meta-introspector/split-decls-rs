use serde::{Deserialize, Serialize};
use std::collections::HashMap;
impl From<(FluentResource, Vec<ParserError>)> for TranslationBundleError {
    fn from((_, errs): (FluentResource, Vec<ParserError>)) -> Self {
        TranslationBundleError::ParseFtl(
            errs.into_iter()
                .next()
                .expect("failed ftl parse with no errors"),
        )
    }
}

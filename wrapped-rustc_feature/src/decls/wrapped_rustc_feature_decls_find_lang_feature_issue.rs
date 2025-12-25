use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn find_lang_feature_issue(feature: Symbol) -> Option<NonZero<u32>> {
    if let Some(f) = UNSTABLE_LANG_FEATURES.iter().find(|f| f.name == feature) {
        return f.issue;
    }
    if let Some(f) = ACCEPTED_LANG_FEATURES.iter().find(|f| f.name == feature) {
        return f.issue;
    }
    if let Some(f) = REMOVED_LANG_FEATURES.iter().find(|f| f.feature.name == feature) {
        return f.feature.issue;
    }
    panic!("feature `{feature}` is not declared anywhere");
}

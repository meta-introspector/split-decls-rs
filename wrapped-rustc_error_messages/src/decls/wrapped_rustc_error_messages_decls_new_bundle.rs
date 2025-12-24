use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn new_bundle(locales: Vec<LanguageIdentifier>) -> FluentBundle {
    IntoDynSyncSend(fluent_bundle::bundle::FluentBundle::new_concurrent(locales))
}

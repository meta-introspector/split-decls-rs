use serde::{Deserialize, Serialize};
use std::collections::HashMap;
fn icu_locale_from_unic_langid(lang: LanguageIdentifier) -> Option<icu_locale::Locale> {
    icu_locale::Locale::try_from_str(&lang.to_string()).ok()
}

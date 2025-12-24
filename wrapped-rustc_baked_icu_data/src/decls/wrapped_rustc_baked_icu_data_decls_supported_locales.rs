use serde::{Deserialize, Serialize};
use std::collections::HashMap;
pub mod supported_locales {
    pub const EN: icu_locale::Locale = icu_locale::locale!("en");
    pub const ES: icu_locale::Locale = icu_locale::locale!("es");
    pub const FR: icu_locale::Locale = icu_locale::locale!("fr");
    pub const IT: icu_locale::Locale = icu_locale::locale!("it");
    pub const JA: icu_locale::Locale = icu_locale::locale!("ja");
    pub const PT: icu_locale::Locale = icu_locale::locale!("pt");
    pub const RU: icu_locale::Locale = icu_locale::locale!("ru");
    pub const TR: icu_locale::Locale = icu_locale::locale!("tr");
    pub const ZH_HANS: icu_locale::Locale = icu_locale::locale!("zh-Hans");
    pub const ZH_HANT: icu_locale::Locale = icu_locale::locale!("zh-Hant");
}

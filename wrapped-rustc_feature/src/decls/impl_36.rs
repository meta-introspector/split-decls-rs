macro_rules! deps {
    () => {
        Feature!();
        Features!();
        EnabledLangFeature!();
        EnabledLibFeature!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl Features { # [doc = " `since` should be set for stable features that are nevertheless enabled with a `#[feature]`"] # [doc = " attribute, indicating since when they are stable."] pub fn set_enabled_lang_feature (& mut self , lang_feat : EnabledLangFeature) { self . enabled_lang_features . push (lang_feat) ; self . enabled_features . insert (lang_feat . gate_name) ; } pub fn set_enabled_lib_feature (& mut self , lib_feat : EnabledLibFeature) { self . enabled_lib_features . push (lib_feat) ; self . enabled_features . insert (lib_feat . gate_name) ; } # [doc = " Returns a list of [`EnabledLangFeature`] with info about:"] # [doc = ""] # [doc = " - Feature gate name."] # [doc = " - The span of the `#[feature]` attribute."] # [doc = " - For stable language features, version info for when it was stabilized."] pub fn enabled_lang_features (& self) -> & Vec < EnabledLangFeature > { & self . enabled_lang_features } pub fn enabled_lib_features (& self) -> & Vec < EnabledLibFeature > { & self . enabled_lib_features } pub fn enabled_features (& self) -> & FxHashSet < Symbol > { & self . enabled_features } # [doc = " Is the given feature enabled (via `#[feature(...)]`)?"] pub fn enabled (& self , feature : Symbol) -> bool { self . enabled_features . contains (& feature) } }
    };
}

impl_36!()
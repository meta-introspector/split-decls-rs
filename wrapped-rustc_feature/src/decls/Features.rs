macro_rules! deps {
    () => {
        EnabledLangFeature!();
        EnabledLibFeature!();
    };
}

macro_rules! Features {
    () => {
        deps!();
        # [doc = " A set of features to be used by later passes."] # [doc = ""] # [doc = " There are two ways to check if a language feature `foo` is enabled:"] # [doc = " - Directly with the `foo` method, e.g. `if tcx.features().foo() { ... }`."] # [doc = " - With the `enabled` method, e.g. `if tcx.features.enabled(sym::foo) { ... }`."] # [doc = ""] # [doc = " The former is preferred. `enabled` should only be used when the feature symbol is not a"] # [doc = " constant, e.g. a parameter, or when the feature is a library feature."] # [derive (Clone , Default , Debug)] pub struct Features { # [doc = " `#![feature]` attrs for language features, for error reporting."] enabled_lang_features : Vec < EnabledLangFeature > , # [doc = " `#![feature]` attrs for non-language (library) features."] enabled_lib_features : Vec < EnabledLibFeature > , # [doc = " `enabled_lang_features` + `enabled_lib_features`."] enabled_features : FxHashSet < Symbol > , }
    };
}

Features!();
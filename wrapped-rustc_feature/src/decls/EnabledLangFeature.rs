macro_rules! EnabledLangFeature {
    () => {
        # [doc = " Information about an enabled language feature."] # [derive (Debug , Copy , Clone)] pub struct EnabledLangFeature { # [doc = " Name of the feature gate guarding the language feature."] pub gate_name : Symbol , # [doc = " Span of the `#[feature(...)]` attribute."] pub attr_sp : Span , # [doc = " If the lang feature is stable, the version number when it was stabilized."] pub stable_since : Option < Symbol > , }
    };
}

EnabledLangFeature!();
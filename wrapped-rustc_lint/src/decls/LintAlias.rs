macro_rules! LintAlias {
    () => {
        struct LintAlias { name : & 'static str , # [doc = " Whether deprecation warnings should be suppressed for this alias."] silent : bool , }
    };
}

LintAlias!();
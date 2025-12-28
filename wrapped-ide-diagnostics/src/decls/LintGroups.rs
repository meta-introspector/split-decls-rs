macro_rules! LintGroups {
    () => {
        # [derive (Debug)] struct LintGroups { groups : & 'static [& 'static str] , inside_warnings : bool , }
    };
}

LintGroups!();
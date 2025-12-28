macro_rules! deps {
    () => {
        ScriptWithExtensions!();
        Script!();
    };
}

macro_rules! tests {
    () => {
        deps!();
        # [cfg (test)] mod tests { use super :: * ; # [test] # [doc = " Regression test for https://github.com/unicode-org/icu4x/issues/6041"] fn test_scx_regression_6041 () { let scripts = ScriptWithExtensions :: new () . get_script_extensions_val ('\u{2bc}') . iter () . collect :: < Vec < _ > > () ; assert_eq ! (scripts , [Script :: Bengali , Script :: Cyrillic , Script :: Devanagari , Script :: Latin , Script :: Thai , Script :: Lisu , Script :: Toto]) ; } }
    };
}

tests!();
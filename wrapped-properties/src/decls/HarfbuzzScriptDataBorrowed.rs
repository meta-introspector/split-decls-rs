macro_rules! deps {
    () => {
        PropertyNamesShortBorrowed!();
        Script!();
        CodePointMapDataBorrowed!();
    };
}

macro_rules! HarfbuzzScriptDataBorrowed {
    () => {
        deps!();
        # [doc = " Harfbuzz data for the [`ScriptFunc`] implementation"] # [doc = ""] # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] # [derive (Debug)] pub struct HarfbuzzScriptDataBorrowed < 'a > { script : CodePointMapDataBorrowed < 'a , Script > , script_names : PropertyNamesShortBorrowed < 'a , Script > , }
    };
}

HarfbuzzScriptDataBorrowed!();
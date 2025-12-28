macro_rules! deps {
    () => {
        CodePointMapData!();
        PropertyNamesShort!();
        Script!();
    };
}

macro_rules! HarfbuzzScriptData {
    () => {
        deps!();
        # [doc = " Harfbuzz data for the [`ScriptFunc`] implementation"] # [doc = ""] # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] # [derive (Debug)] pub struct HarfbuzzScriptData { script : CodePointMapData < Script > , script_names : PropertyNamesShort < Script > , }
    };
}

HarfbuzzScriptData!()
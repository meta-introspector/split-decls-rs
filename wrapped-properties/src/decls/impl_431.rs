macro_rules! deps {
    () => {
        HarfbuzzScriptData!();
    };
}

macro_rules! impl_431 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl ScriptFunc for HarfbuzzScriptData { fn script (& self , ch : char) -> [u8 ; 4] { ScriptFunc :: script (& self . as_borrowed () , ch) } }
    };
}

impl_431!();
macro_rules! deps {
    () => {
        HarfbuzzScriptDataBorrowed!();
    };
}

macro_rules! impl_430 {
    () => {
        deps!();
        # [doc = " ✨ *Enabled with the `harfbuzz_traits` Cargo feature.*"] impl ScriptFunc for HarfbuzzScriptDataBorrowed < '_ > { fn script (& self , ch : char) -> [u8 ; 4] { let script = self . script . get (ch) ; self . script_names . get_locale_script (script) . unwrap_or (icu_locale_core :: subtags :: script ! ("Zzzz")) . into_raw () } }
    };
}

impl_430!();
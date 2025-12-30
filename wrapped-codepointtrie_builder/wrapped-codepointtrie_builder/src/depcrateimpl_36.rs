// Generated macro for impl_36 (impl)
macro_rules! Depcrateimpl_36 {
() => {
// Module: crate
// Provides: {"impl_36"}
// Dependencies: {}
impl < T > CodePointTrieBuilder < '_ , T > where T : TrieValue , { # [doc = " Build the [`CodePointTrie`]."] # [doc = ""] # [doc = " Under the hood, this function runs ICU4C code compiled into WASM,"] # [doc = " or links natively to ICU4C as specified by the `ICU4C_LIB_PATH` env var"] # [doc = ""] # [doc = " ✨ *Enabled with either the `wasm` or the `icu4c` Cargo feature.*"] # [doc = ""] # [doc = " [`CodePointTrie`]: icu_collections::codepointtrie::CodePointTrie"] # [cfg (any (feature = "wasm" , feature = "icu4c"))] pub fn build (self) -> icu_collections :: codepointtrie :: CodePointTrie < 'static , T > { # [cfg (feature = "wasm")] { wasm :: run_wasmi_ucptrie_wrap (& self) } # [cfg (all (feature = "icu4c" , not (feature = "wasm")))] { native :: run_native (& self) } } }
};
}

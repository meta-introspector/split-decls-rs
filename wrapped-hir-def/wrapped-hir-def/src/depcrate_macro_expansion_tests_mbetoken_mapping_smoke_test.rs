// Generated macro for token_mapping_smoke_test (function)
macro_rules! Depcrate_macro_expansion_tests_mbetoken_mapping_smoke_test {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"token_mapping_smoke_test"}
// Dependencies: {}
# [test] fn token_mapping_smoke_test () { check (r#"
macro_rules! f {
    ( struct $ident:ident ) => {
        struct $ident {
            map: ::std::collections::HashSet<()>,
        }
    };
}

// +spans+syntaxctxt
f!(struct MyTraitMap2);
"# , expect ! [[r#"
macro_rules! f {
    ( struct $ident:ident ) => {
        struct $ident {
            map: ::std::collections::HashSet<()>,
        }
    };
}

struct#0:MacroRules[BE8F, 0]@58..64#14336# MyTraitMap2#0:MacroCall[BE8F, 0]@31..42#ROOT2024# {#0:MacroRules[BE8F, 0]@72..73#14336#
    map#0:MacroRules[BE8F, 0]@86..89#14336#:#0:MacroRules[BE8F, 0]@89..90#14336# #0:MacroRules[BE8F, 0]@89..90#14336#::#0:MacroRules[BE8F, 0]@91..93#14336#std#0:MacroRules[BE8F, 0]@93..96#14336#::#0:MacroRules[BE8F, 0]@96..98#14336#collections#0:MacroRules[BE8F, 0]@98..109#14336#::#0:MacroRules[BE8F, 0]@109..111#14336#HashSet#0:MacroRules[BE8F, 0]@111..118#14336#<#0:MacroRules[BE8F, 0]@118..119#14336#(#0:MacroRules[BE8F, 0]@119..120#14336#)#0:MacroRules[BE8F, 0]@120..121#14336#>#0:MacroRules[BE8F, 0]@121..122#14336#,#0:MacroRules[BE8F, 0]@122..123#14336#
}#0:MacroRules[BE8F, 0]@132..133#14336#
"#]] ,) ; }
};
}

// Generated macro for float_attribute_mapping (function)
macro_rules! Depcrate_macro_expansion_tests_proc_macrosfloat_attribute_mapping {
() => {
// Module: crate::macro_expansion_tests::proc_macros
// Provides: {"float_attribute_mapping"}
// Dependencies: {}
# [test] fn float_attribute_mapping () { check (r#"
//- proc_macros: identity
//+spans+syntaxctxt
#[proc_macros::identity]
fn foo(&self) {
    self.0. 1;
}
"# , expect ! [[r#"
//+spans+syntaxctxt
#[proc_macros::identity]
fn foo(&self) {
    self.0. 1;
}

fn#0:Fn[8A31, 0]@45..47#ROOT2024# foo#0:Fn[8A31, 0]@48..51#ROOT2024#(#0:Fn[8A31, 0]@51..52#ROOT2024#&#0:Fn[8A31, 0]@52..53#ROOT2024#self#0:Fn[8A31, 0]@53..57#ROOT2024# )#0:Fn[8A31, 0]@57..58#ROOT2024# {#0:Fn[8A31, 0]@59..60#ROOT2024#
    self#0:Fn[8A31, 0]@65..69#ROOT2024# .#0:Fn[8A31, 0]@69..70#ROOT2024#0#0:Fn[8A31, 0]@70..71#ROOT2024#.#0:Fn[8A31, 0]@71..72#ROOT2024#1#0:Fn[8A31, 0]@73..74#ROOT2024#;#0:Fn[8A31, 0]@74..75#ROOT2024#
}#0:Fn[8A31, 0]@76..77#ROOT2024#"#]] ,) ; }
};
}

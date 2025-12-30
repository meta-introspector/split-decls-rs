// Generated macro for token_mapping_floats (function)
macro_rules! Depcrate_macro_expansion_tests_mbetoken_mapping_floats {
() => {
// Module: crate::macro_expansion_tests::mbe
// Provides: {"token_mapping_floats"}
// Dependencies: {}
# [test] fn token_mapping_floats () { check (r#"
// +spans+syntaxctxt
macro_rules! f {
    ($($tt:tt)*) => {
        $($tt)*
    };
}

// +spans+syntaxctxt
f! {
    fn main() {
        1;
        1.0;
        ((1,),).0.0;
        let x = 1;
    }
}


"# , expect ! [[r#"
// +spans+syntaxctxt
macro_rules! f {
    ($($tt:tt)*) => {
        $($tt)*
    };
}

fn#0:MacroCall[BE8F, 0]@30..32#ROOT2024# main#0:MacroCall[BE8F, 0]@33..37#ROOT2024#(#0:MacroCall[BE8F, 0]@37..38#ROOT2024#)#0:MacroCall[BE8F, 0]@38..39#ROOT2024# {#0:MacroCall[BE8F, 0]@40..41#ROOT2024#
    1#0:MacroCall[BE8F, 0]@50..51#ROOT2024#;#0:MacroCall[BE8F, 0]@51..52#ROOT2024#
    1.0#0:MacroCall[BE8F, 0]@61..64#ROOT2024#;#0:MacroCall[BE8F, 0]@64..65#ROOT2024#
    (#0:MacroCall[BE8F, 0]@74..75#ROOT2024#(#0:MacroCall[BE8F, 0]@75..76#ROOT2024#1#0:MacroCall[BE8F, 0]@76..77#ROOT2024#,#0:MacroCall[BE8F, 0]@77..78#ROOT2024# )#0:MacroCall[BE8F, 0]@78..79#ROOT2024#,#0:MacroCall[BE8F, 0]@79..80#ROOT2024# )#0:MacroCall[BE8F, 0]@80..81#ROOT2024#.#0:MacroCall[BE8F, 0]@81..82#ROOT2024#0#0:MacroCall[BE8F, 0]@82..85#ROOT2024#.#0:MacroCall[BE8F, 0]@82..85#ROOT2024#0#0:MacroCall[BE8F, 0]@82..85#ROOT2024#;#0:MacroCall[BE8F, 0]@85..86#ROOT2024#
    let#0:MacroCall[BE8F, 0]@95..98#ROOT2024# x#0:MacroCall[BE8F, 0]@99..100#ROOT2024# =#0:MacroCall[BE8F, 0]@101..102#ROOT2024# 1#0:MacroCall[BE8F, 0]@103..104#ROOT2024#;#0:MacroCall[BE8F, 0]@104..105#ROOT2024#
}#0:MacroCall[BE8F, 0]@110..111#ROOT2024#


"#]] ,) ; }
};
}

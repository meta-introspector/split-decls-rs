macro_rules! token_mapping_across_files {
    () => {
        # [test] fn token_mapping_across_files () { check (r#"
//- /lib.rs
#[macro_use]
mod foo;

mk_struct/*+spans+syntaxctxt*/!(Foo with u32);
//- /foo.rs
macro_rules! mk_struct {
    ($foo:ident with $ty:ty) => { struct $foo($ty); }
}
"# , expect ! [[r#"
#[macro_use]
mod foo;

struct#1:MacroRules[DB0C, 0]@59..65#14336# Foo#0:MacroCall[DB0C, 0]@32..35#ROOT2024#(#1:MacroRules[DB0C, 0]@70..71#14336#u32#0:MacroCall[DB0C, 0]@41..44#ROOT2024#)#1:MacroRules[DB0C, 0]@74..75#14336#;#1:MacroRules[DB0C, 0]@75..76#14336#
"#]] ,) ; }
    };
}

token_mapping_across_files!()
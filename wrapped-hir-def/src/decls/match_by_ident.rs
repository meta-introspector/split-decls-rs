macro_rules! match_by_ident {
    () => {
        # [test] fn match_by_ident () { check (r#"
macro_rules! m {
    ($i:ident) => ( mod $i {} );
    (spam $i:ident) => ( fn $i() {} );
    (eggs $i:ident) => ( struct $i; )
}
m! { foo }
m! { spam bar }
m! { eggs Baz }
"# , expect ! [[r#"
macro_rules! m {
    ($i:ident) => ( mod $i {} );
    (spam $i:ident) => ( fn $i() {} );
    (eggs $i:ident) => ( struct $i; )
}
mod foo {}
fn bar() {}
struct Baz;
"#]] ,) ; }
    };
}

match_by_ident!();
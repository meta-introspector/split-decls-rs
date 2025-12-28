macro_rules! semicolon_does_not_glue {
    () => {
        # [test] fn semicolon_does_not_glue () { check (r#"
macro_rules! bug {
    ($id: expr) => {
        true
    };
    ($id: expr; $($attr: ident),*) => {
        true
    };
    ($id: expr; $($attr: ident),*; $norm: expr) => {
        true
    };
    ($id: expr; $($attr: ident),*;; $print: expr) => {
        true
    };
    ($id: expr; $($attr: ident),*; $norm: expr; $print: expr) => {
        true
    };
}
fn f() {
    let _ = bug!(a;;;test);
}
    "# , expect ! [[r#"
macro_rules! bug {
    ($id: expr) => {
        true
    };
    ($id: expr; $($attr: ident),*) => {
        true
    };
    ($id: expr; $($attr: ident),*; $norm: expr) => {
        true
    };
    ($id: expr; $($attr: ident),*;; $print: expr) => {
        true
    };
    ($id: expr; $($attr: ident),*; $norm: expr; $print: expr) => {
        true
    };
}
fn f() {
    let _ = true;
}
    "#]] ,) ; }
    };
}

semicolon_does_not_glue!();
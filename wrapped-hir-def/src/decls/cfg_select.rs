macro_rules! cfg_select {
    () => {
        # [test] fn cfg_select () { check (r#"
#[rustc_builtin_macro]
pub macro cfg_select($($tt:tt)*) {}

cfg_select! {
    false => { fn false_1() {} }
    any(false, true) => { fn true_1() {} }
}

cfg_select! {
    false => { fn false_2() {} }
    _ => { fn true_2() {} }
}

cfg_select! {
    false => { fn false_3() {} }
}

cfg_select! {
    false
}

cfg_select! {
    false =>
}

    "# , expect ! [[r#"
#[rustc_builtin_macro]
pub macro cfg_select($($tt:tt)*) {}

fn true_1() {}

fn true_2() {}

/* error: none of the predicates in this `cfg_select` evaluated to true */

/* error: expected `=>` after cfg expression */

/* error: expected a token tree after `=>` */

    "#]] ,) ; }
    };
}

cfg_select!()
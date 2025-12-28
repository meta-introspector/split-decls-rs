macro_rules! distribute {
    () => {
        # [test] fn distribute () { check_dnf ("#![cfg(all(any(a, b), c))]" , expect ! [[r#"#![cfg(any(all(a, c), all(b, c)))]"#]]) ; check_dnf ("#![cfg(all(c, any(a, b)))]" , expect ! [[r#"#![cfg(any(all(c, a), all(c, b)))]"#]]) ; check_dnf ("#![cfg(all(any(a, b), any(c, d)))]" , expect ! [[r#"#![cfg(any(all(a, c), all(a, d), all(b, c), all(b, d)))]"#]] ,) ; check_dnf ("#![cfg(all(any(a, b, c), any(d, e, f), g))]" , expect ! [[r#"#![cfg(any(all(a, d, g), all(a, e, g), all(a, f, g), all(b, d, g), all(b, e, g), all(b, f, g), all(c, d, g), all(c, e, g), all(c, f, g)))]"#]] ,) ; }
    };
}

distribute!()
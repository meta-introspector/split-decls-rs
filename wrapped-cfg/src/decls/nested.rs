macro_rules! nested {
    () => {
        # [test] fn nested () { check_dnf ("#![cfg(all(any(a), not(all(any(b)))))]" , expect ! [[r#"#![cfg(all(a, not(b)))]"#]]) ; check_dnf ("#![cfg(any(any(a, b)))]" , expect ! [[r#"#![cfg(any(a, b))]"#]]) ; check_dnf ("#![cfg(not(any(any(a, b))))]" , expect ! [[r#"#![cfg(all(not(a), not(b)))]"#]]) ; check_dnf ("#![cfg(all(all(a, b)))]" , expect ! [[r#"#![cfg(all(a, b))]"#]]) ; check_dnf ("#![cfg(not(all(all(a, b))))]" , expect ! [[r#"#![cfg(any(not(a), not(b)))]"#]]) ; }
    };
}

nested!();
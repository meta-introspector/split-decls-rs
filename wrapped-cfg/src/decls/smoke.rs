macro_rules! smoke {
    () => {
        # [test] fn smoke () { check_dnf ("#![cfg(test)]" , expect ! [[r#"#![cfg(test)]"#]]) ; check_dnf ("#![cfg(not(test))]" , expect ! [[r#"#![cfg(not(test))]"#]]) ; check_dnf ("#![cfg(not(not(test)))]" , expect ! [[r#"#![cfg(test)]"#]]) ; check_dnf ("#![cfg(all(a, b))]" , expect ! [[r#"#![cfg(all(a, b))]"#]]) ; check_dnf ("#![cfg(any(a, b))]" , expect ! [[r#"#![cfg(any(a, b))]"#]]) ; check_dnf ("#![cfg(not(a))]" , expect ! [[r#"#![cfg(not(a))]"#]]) ; }
    };
}

smoke!()
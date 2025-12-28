macro_rules! test_underscore_flavors {
    () => {
        # [test] fn test_underscore_flavors () { check (r#"
macro_rules! m1 { ($a:ty) => { ok!(); } }
m1![_];

macro_rules! m2 { ($a:lifetime) => { ok!(); } }
m2!['_];
"# , expect ! [[r#"
macro_rules! m1 { ($a:ty) => { ok!(); } }
ok!();

macro_rules! m2 { ($a:lifetime) => { ok!(); } }
ok!();
"#]] ,) ; }
    };
}

test_underscore_flavors!();
macro_rules! test_single_item {
    () => {
        # [test] fn test_single_item () { check (r#"
macro_rules! m { ($i:item) => ( $i ) }
m! { mod c {} }
"# , expect ! [[r#"
macro_rules! m { ($i:item) => ( $i ) }
mod c {}
"#]] ,) }
    };
}

test_single_item!()
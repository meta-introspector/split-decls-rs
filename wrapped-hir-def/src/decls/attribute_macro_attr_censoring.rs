macro_rules! attribute_macro_attr_censoring {
    () => {
        # [test] fn attribute_macro_attr_censoring () { cov_mark :: check ! (attribute_macro_attr_censoring) ; check (r#"
//- proc_macros: identity
#[attr1] #[proc_macros::identity] #[attr2]
struct S;
"# , expect ! [[r#"
#[attr1] #[proc_macros::identity] #[attr2]
struct S;

#[attr1]
#[attr2] struct S;"#]] ,) ; }
    };
}

attribute_macro_attr_censoring!();
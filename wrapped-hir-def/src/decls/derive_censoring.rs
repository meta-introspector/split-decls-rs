macro_rules! derive_censoring {
    () => {
        # [test] fn derive_censoring () { cov_mark :: check ! (derive_censoring) ; check (r#"
//- proc_macros: derive_identity
//- minicore:derive
#[attr1]
#[derive(Foo)]
#[derive(proc_macros::DeriveIdentity)]
#[derive(Bar)]
#[attr2]
struct S;
"# , expect ! [[r#"
#[attr1]
#[derive(Foo)]
#[derive(proc_macros::DeriveIdentity)]
#[derive(Bar)]
#[attr2]
struct S;

#[attr1]
#[derive(Bar)]
#[attr2] struct S;"#]] ,) ; }
    };
}

derive_censoring!();
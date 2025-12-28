macro_rules! replace_type {
    () => {
        # [test] fn replace_type () { assert_ssr_transform ("Result<(), $a> ==>> Option<$a>" , "struct Result<T, E> {} struct Option<T> {} fn f1() -> Result<(), Vec<Error>> {foo()}" , expect ! [["struct Result<T, E> {} struct Option<T> {} fn f1() -> Option<Vec<Error>> {foo()}"]] ,) ; assert_ssr_transform ("dyn Trait<$a> ==>> DynTrait<$a>" , r#"
trait Trait<T> {}
struct DynTrait<T> {}
fn f1() -> dyn Trait<Vec<Error>> {foo()}
"# , expect ! [[r#"
trait Trait<T> {}
struct DynTrait<T> {}
fn f1() -> DynTrait<Vec<Error>> {foo()}
"#]] ,) ; }
    };
}

replace_type!()
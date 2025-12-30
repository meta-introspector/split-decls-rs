// Generated macro for test_partial_eq_expand (function)
macro_rules! Depcrate_macro_expansion_tests_builtin_derive_macrotest_partial_eq_expand {
() => {
// Module: crate::macro_expansion_tests::builtin_derive_macro
// Provides: {"test_partial_eq_expand"}
// Dependencies: {}
# [test] fn test_partial_eq_expand () { check (r#"
//- minicore: derive, eq
#[derive(PartialEq, Eq)]
enum Command {
    Move { x: i32, y: i32 },
    Do(&'static str),
    Jump,
}
"# , expect ! [[r#"
#[derive(PartialEq, Eq)]
enum Command {
    Move { x: i32, y: i32 },
    Do(&'static str),
    Jump,
}

impl <> $crate::cmp::PartialEq for Command< > where {
    fn eq(&self , other: &Self ) -> bool {
        match (self , other) {
            (Command::Move {
                x: x_self, y: y_self,
            }
            , Command::Move {
                x: x_other, y: y_other,
            }
            )=>x_self.eq(x_other) && y_self.eq(y_other), (Command::Do(f0_self, ), Command::Do(f0_other, ))=>f0_self.eq(f0_other), (Command::Jump, Command::Jump)=>true , _unused=>false
        }
    }
}
impl <> $crate::cmp::Eq for Command< > where {}"#]] ,) ; }
};
}

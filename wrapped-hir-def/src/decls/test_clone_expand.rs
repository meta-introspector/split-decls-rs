macro_rules! test_clone_expand {
    () => {
        # [test] fn test_clone_expand () { check (r#"
//- minicore: derive, clone
#[derive(Clone)]
enum Command<A, B> {
    Move { x: A, y: B },
    Do(&'static str),
    Jump,
}
"# , expect ! [[r#"
#[derive(Clone)]
enum Command<A, B> {
    Move { x: A, y: B },
    Do(&'static str),
    Jump,
}

impl <A: $crate::clone::Clone, B: $crate::clone::Clone, > $crate::clone::Clone for Command<A, B, > where {
    fn clone(&self ) -> Self {
        match self {
            Command::Move {
                x: x, y: y,
            }
            =>Command::Move {
                x: x.clone(), y: y.clone(),
            }
            , Command::Do(f0, )=>Command::Do(f0.clone(), ), Command::Jump=>Command::Jump,
        }
    }
}"#]] ,) ; }
    };
}

test_clone_expand!();
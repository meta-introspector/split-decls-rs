macro_rules! test_debug_expand {
    () => {
        # [test] fn test_debug_expand () { check (r#"
//- minicore: derive, fmt
use core::fmt::Debug;

#[derive(Debug)]
enum Command {
    Move { x: i32, y: i32 },
    Do(&'static str),
    Jump,
}
"# , expect ! [[r#"
use core::fmt::Debug;

#[derive(Debug)]
enum Command {
    Move { x: i32, y: i32 },
    Do(&'static str),
    Jump,
}

impl <> $crate::fmt::Debug for Command< > where {
    fn fmt(&self , f: &mut $crate::fmt::Formatter) -> $crate::fmt::Result {
        match self {
            Command::Move {
                x: x, y: y,
            }
            =>f.debug_struct("Move").field("x", &x).field("y", &y).finish(), Command::Do(f0, )=>f.debug_tuple("Do").field(&f0).finish(), Command::Jump=>f.write_str("Jump"),
        }
    }
}"#]] ,) ; }
    };
}

test_debug_expand!();
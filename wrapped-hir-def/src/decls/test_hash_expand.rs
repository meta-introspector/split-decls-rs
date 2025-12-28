macro_rules! test_hash_expand {
    () => {
        # [test] fn test_hash_expand () { check (r#"
//- minicore: derive, hash
use core::hash::Hash;

#[derive(Hash)]
struct Foo {
    x: i32,
    y: u64,
    z: (i32, u64),
}
"# , expect ! [[r#"
use core::hash::Hash;

#[derive(Hash)]
struct Foo {
    x: i32,
    y: u64,
    z: (i32, u64),
}

impl <> $crate::hash::Hash for Foo< > where {
    fn hash<H: $crate::hash::Hasher>(&self , ra_expand_state: &mut H) {
        match self {
            Foo {
                x: x, y: y, z: z,
            }
            => {
                x.hash(ra_expand_state);
                y.hash(ra_expand_state);
                z.hash(ra_expand_state);
            }
            ,
        }
    }
}"#]] ,) ; check (r#"
//- minicore: derive, hash
use core::hash::Hash;

#[derive(Hash)]
enum Command {
    Move { x: i32, y: i32 },
    Do(&'static str),
    Jump,
}
"# , expect ! [[r#"
use core::hash::Hash;

#[derive(Hash)]
enum Command {
    Move { x: i32, y: i32 },
    Do(&'static str),
    Jump,
}

impl <> $crate::hash::Hash for Command< > where {
    fn hash<H: $crate::hash::Hasher>(&self , ra_expand_state: &mut H) {
        $crate::mem::discriminant(self ).hash(ra_expand_state);
        match self {
            Command::Move {
                x: x, y: y,
            }
            => {
                x.hash(ra_expand_state);
                y.hash(ra_expand_state);
            }
            , Command::Do(f0, )=> {
                f0.hash(ra_expand_state);
            }
            , Command::Jump=> {}
            ,
        }
    }
}"#]] ,) ; }
    };
}

test_hash_expand!()
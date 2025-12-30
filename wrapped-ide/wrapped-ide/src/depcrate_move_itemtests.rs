// Generated macro for tests (module)
macro_rules! Depcrate_move_itemtests {
() => {
// Module: crate::move_item
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: fixture ; use expect_test :: { Expect , expect } ; use crate :: Direction ; fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect , direction : Direction ,) { let (analysis , range) = fixture :: range (ra_fixture) ; let edit = analysis . move_item (range , direction) . unwrap () . unwrap_or_default () ; let mut file = analysis . file_text (range . file_id) . unwrap () . to_string () ; edit . apply (& mut file) ; expect . assert_eq (& file) ; } # [test] fn test_moves_match_arm_up () { check (r#"
fn main() {
    match true {
        true => {
            println!("Hello, world");
        },
        false =>$0$0 {
            println!("Test");
        }
    };
}
"# , expect ! [[r#"
                fn main() {
                    match true {
                        false =>$0 {
                            println!("Test");
                        }
                        true => {
                            println!("Hello, world");
                        },
                    };
                }
            "#]] , Direction :: Up ,) ; } # [test] fn test_moves_match_arm_down () { check (r#"
fn main() {
    match true {
        true =>$0$0 {
            println!("Hello, world");
        },
        false => {
            println!("Test");
        }
    };
}
"# , expect ! [[r#"
                fn main() {
                    match true {
                        false => {
                            println!("Test");
                        }
                        true =>$0 {
                            println!("Hello, world");
                        },
                    };
                }
            "#]] , Direction :: Down ,) ; } # [test] fn test_nowhere_to_move () { check (r#"
fn main() {
    match true {
        true =>$0$0 {
            println!("Hello, world");
        },
        false => {
            println!("Test");
        }
    };
}
"# , expect ! [[r#"
                fn main() {
                    match true {
                        true => {
                            println!("Hello, world");
                        },
                        false => {
                            println!("Test");
                        }
                    };
                }
            "#]] , Direction :: Up ,) ; } # [test] fn test_moves_let_stmt_up () { check (r#"
fn main() {
    let test = 123;
    let test2$0$0 = 456;
}
"# , expect ! [[r#"
                fn main() {
                    let test2$0 = 456;
                    let test = 123;
                }
            "#]] , Direction :: Up ,) ; } # [test] fn test_moves_expr_up () { check (r#"
fn main() {
    println!("Hello, world");
    println!("All I want to say is...");$0$0
}
"# , expect ! [[r#"
                fn main() {
                    println!("All I want to say is...");$0
                    println!("Hello, world");
                }
            "#]] , Direction :: Up ,) ; check (r#"
fn main() {
    println!("Hello, world");

    if true {
        println!("Test");
    }$0$0
}
"# , expect ! [[r#"
                fn main() {
                    if true {
                        println!("Test");
                    }$0

                    println!("Hello, world");
                }
            "#]] , Direction :: Up ,) ; check (r#"
fn main() {
    println!("Hello, world");

    for i in 0..10 {
        println!("Test");
    }$0$0
}
"# , expect ! [[r#"
                fn main() {
                    for i in 0..10 {
                        println!("Test");
                    }$0

                    println!("Hello, world");
                }
            "#]] , Direction :: Up ,) ; check (r#"
fn main() {
    println!("Hello, world");

    loop {
        println!("Test");
    }$0$0
}
"# , expect ! [[r#"
                fn main() {
                    loop {
                        println!("Test");
                    }$0

                    println!("Hello, world");
                }
            "#]] , Direction :: Up ,) ; check (r#"
fn main() {
    println!("Hello, world");

    while true {
        println!("Test");
    }$0$0
}
"# , expect ! [[r#"
                fn main() {
                    while true {
                        println!("Test");
                    }$0

                    println!("Hello, world");
                }
            "#]] , Direction :: Up ,) ; check (r#"
fn main() {
    println!("Hello, world");

    return 123;$0$0
}
"# , expect ! [[r#"
                fn main() {
                    return 123;$0

                    println!("Hello, world");
                }
            "#]] , Direction :: Up ,) ; } # [test] fn test_nowhere_to_move_stmt () { check (r#"
fn main() {
    println!("All I want to say is...");$0$0
    println!("Hello, world");
}
"# , expect ! [[r#"
                fn main() {
                    println!("All I want to say is...");
                    println!("Hello, world");
                }
            "#]] , Direction :: Up ,) ; } # [test] fn test_move_item () { check (r#"
fn main() {}

fn foo() {}$0$0
"# , expect ! [[r#"
                fn foo() {}$0

                fn main() {}
            "#]] , Direction :: Up ,) ; } # [test] fn test_move_impl_up () { check (r#"
struct Yay;

trait Wow {}

impl Wow for Yay $0$0{}
"# , expect ! [[r#"
                struct Yay;

                impl Wow for Yay $0{}

                trait Wow {}
            "#]] , Direction :: Up ,) ; } # [test] fn test_move_use_up () { check (r#"
use std::vec::Vec;
use std::collections::HashMap$0$0;
"# , expect ! [[r#"
                use std::collections::HashMap$0;
                use std::vec::Vec;
            "#]] , Direction :: Up ,) ; } # [test] fn test_moves_match_expr_up () { check (r#"
fn main() {
    let test = 123;

    $0match test {
        456 => {},
        _ => {}
    };$0
}
"# , expect ! [[r#"
                fn main() {
                    match test {
                        456 => {},
                        _ => {}
                    };

                    let test = 123;
                }
            "#]] , Direction :: Up ,) ; } # [test] fn test_moves_param () { check (r#"
fn test(one: i32, two$0$0: u32) {}

fn main() {
    test(123, 456);
}
"# , expect ! [[r#"
                fn test(two$0: u32, one: i32) {}

                fn main() {
                    test(123, 456);
                }
            "#]] , Direction :: Up ,) ; check (r#"
fn f($0$0arg: u8, arg2: u16) {}
"# , expect ! [[r#"
                fn f(arg2: u16, $0arg: u8) {}
            "#]] , Direction :: Down ,) ; } # [test] fn test_moves_arg_up () { check (r#"
fn test(one: i32, two: u32) {}

fn main() {
    test(123, 456$0$0);
}
"# , expect ! [[r#"
                fn test(one: i32, two: u32) {}

                fn main() {
                    test(456$0, 123);
                }
            "#]] , Direction :: Up ,) ; } # [test] fn test_moves_arg_down () { check (r#"
fn test(one: i32, two: u32) {}

fn main() {
    test(123$0$0, 456);
}
"# , expect ! [[r#"
                fn test(one: i32, two: u32) {}

                fn main() {
                    test(456, 123$0);
                }
            "#]] , Direction :: Down ,) ; } # [test] fn test_nowhere_to_move_arg () { check (r#"
fn test(one: i32, two: u32) {}

fn main() {
    test(123$0$0, 456);
}
"# , expect ! [[r#"
                fn test(one: i32, two: u32) {}

                fn main() {
                    test(123, 456);
                }
            "#]] , Direction :: Up ,) ; } # [test] fn test_moves_generic_param_up () { check (r#"
struct Test<A, B$0$0>(A, B);

fn main() {}
"# , expect ! [[r#"
                struct Test<B$0, A>(A, B);

                fn main() {}
            "#]] , Direction :: Up ,) ; } # [test] fn test_moves_generic_arg_up () { check (r#"
struct Test<A, B>(A, B);

fn main() {
    let t = Test::<i32, &str$0$0>(123, "yay");
}
"# , expect ! [[r#"
                struct Test<A, B>(A, B);

                fn main() {
                    let t = Test::<&str$0, i32>(123, "yay");
                }
            "#]] , Direction :: Up ,) ; } # [test] fn test_moves_variant_up () { check (r#"
enum Hello {
    One,
    Two$0$0
}

fn main() {}
"# , expect ! [[r#"
                enum Hello {
                    Two$0,
                    One
                }

                fn main() {}
            "#]] , Direction :: Up ,) ; } # [test] fn test_moves_type_bound_up () { check (r#"
trait One {}

trait Two {}

fn test<T: One + Two$0$0>(t: T) {}

fn main() {}
"# , expect ! [[r#"
                trait One {}

                trait Two {}

                fn test<T: Two$0 + One>(t: T) {}

                fn main() {}
            "#]] , Direction :: Up ,) ; } # [test] fn test_prioritizes_trait_items () { check (r#"
struct Test;

trait Yay {
    type One;

    type Two;

    fn inner();
}

impl Yay for Test {
    type One = i32;

    type Two = u32;

    fn inner() {$0$0
        println!("Mmmm");
    }
}
"# , expect ! [[r#"
                struct Test;

                trait Yay {
                    type One;

                    type Two;

                    fn inner();
                }

                impl Yay for Test {
                    type One = i32;

                    fn inner() {$0
                        println!("Mmmm");
                    }

                    type Two = u32;
                }
            "#]] , Direction :: Up ,) ; } # [test] fn test_weird_nesting () { check (r#"
fn test() {
    mod hello {
        fn inner() {}
    }

    mod hi {$0$0
        fn inner() {}
    }
}
"# , expect ! [[r#"
                fn test() {
                    mod hi {$0
                        fn inner() {}
                    }

                    mod hello {
                        fn inner() {}
                    }
                }
            "#]] , Direction :: Up ,) ; } # [test] fn test_cursor_at_item_start () { check (r#"
$0$0#[derive(Debug)]
enum FooBar {
    Foo,
    Bar,
}

fn main() {}
"# , expect ! [[r##"
                fn main() {}

                $0#[derive(Debug)]
                enum FooBar {
                    Foo,
                    Bar,
                }
            "##]] , Direction :: Down ,) ; check (r#"
$0$0enum FooBar {
    Foo,
    Bar,
}

fn main() {}
"# , expect ! [[r#"
                fn main() {}

                $0enum FooBar {
                    Foo,
                    Bar,
                }
            "#]] , Direction :: Down ,) ; check (r#"
struct Test;

trait SomeTrait {}

$0$0impl SomeTrait for Test {}

fn main() {}
"# , expect ! [[r#"
                struct Test;

                $0impl SomeTrait for Test {}

                trait SomeTrait {}

                fn main() {}
            "#]] , Direction :: Up ,) ; } # [test] fn test_cursor_at_item_end () { check (r#"
enum FooBar {
    Foo,
    Bar,
}$0$0

fn main() {}
"# , expect ! [[r#"
                fn main() {}

                enum FooBar {
                    Foo,
                    Bar,
                }$0
            "#]] , Direction :: Down ,) ; check (r#"
struct Test;

trait SomeTrait {}

impl SomeTrait for Test {}$0$0

fn main() {}
"# , expect ! [[r#"
                struct Test;

                impl SomeTrait for Test {}$0

                trait SomeTrait {}

                fn main() {}
            "#]] , Direction :: Up ,) ; } # [test] fn handles_empty_file () { check (r#"$0$0"# , expect ! [[r#""#]] , Direction :: Up) ; } }
};
}

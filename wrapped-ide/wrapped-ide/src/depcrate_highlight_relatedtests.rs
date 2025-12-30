// Generated macro for tests (module)
macro_rules! Depcrate_highlight_relatedtests {
() => {
// Module: crate::highlight_related
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use itertools :: Itertools ; use crate :: fixture ; use super :: * ; const ENABLED_CONFIG : HighlightRelatedConfig = HighlightRelatedConfig { break_points : true , exit_points : true , references : true , closure_captures : true , yield_points : true , branch_exit_points : true , } ; # [track_caller] fn check (# [rust_analyzer :: rust_fixture] ra_fixture : & str) { check_with_config (ra_fixture , ENABLED_CONFIG) ; } # [track_caller] fn check_with_config (# [rust_analyzer :: rust_fixture] ra_fixture : & str , config : HighlightRelatedConfig ,) { let (analysis , pos , annotations) = fixture :: annotations (ra_fixture) ; let hls = analysis . highlight_related (config , pos) . unwrap () . unwrap_or_default () ; let mut expected = annotations . into_iter () . map (| (r , access) | (r . range , access)) . collect :: < Vec < _ > > () ; let mut actual : Vec < (TextRange , String) > = hls . into_iter () . map (| hl | { (hl . range , hl . category . iter_names () . map (| (name , _flag) | name . to_lowercase ()) . join (",") ,) }) . collect () ; actual . sort_by_key (| (range , _) | range . start ()) ; expected . sort_by_key (| (range , _) | range . start ()) ; assert_eq ! (expected , actual) ; } # [test] fn test_hl_unsafe_block () { check (r#"
fn foo() {
    unsafe fn this_is_unsafe_function() {}

    unsa$0fe {
  //^^^^^^
        let raw_ptr = &42 as *const i32;
        let val = *raw_ptr;
                //^^^^^^^^

        let mut_ptr = &mut 5 as *mut i32;
        *mut_ptr = 10;
      //^^^^^^^^

        this_is_unsafe_function();
      //^^^^^^^^^^^^^^^^^^^^^^^^^
    }

}
"# ,) ; } # [test] fn test_hl_tuple_fields () { check (r#"
struct Tuple(u32, u32);

fn foo(t: Tuple) {
    t.0$0;
   // ^ read
    t.0;
   // ^ read
}
"# ,) ; } # [test] fn test_hl_module () { check (r#"
//- /lib.rs
mod foo$0;
 // ^^^
//- /foo.rs
struct Foo;
"# ,) ; } # [test] fn test_hl_self_in_crate_root () { check (r#"
use crate$0;
  //^^^^^ import
use self;
  //^^^^ import
mod __ {
    use super;
      //^^^^^ import
}
"# ,) ; check (r#"
//- /main.rs crate:main deps:lib
use lib$0;
  //^^^ import
//- /lib.rs crate:lib
"# ,) ; } # [test] fn test_hl_self_in_module () { check (r#"
//- /lib.rs
mod foo;
//- /foo.rs
use self$0;
 // ^^^^ import
"# ,) ; } # [test] fn test_hl_local () { check (r#"
fn foo() {
    let mut bar = 3;
         // ^^^ write
    bar$0;
 // ^^^ read
}
"# ,) ; } # [test] fn test_hl_local_in_attr () { check (r#"
//- proc_macros: identity
#[proc_macros::identity]
fn foo() {
    let mut bar = 3;
         // ^^^ write
    bar$0;
 // ^^^ read
}
"# ,) ; } # [test] fn test_multi_macro_usage () { check (r#"
macro_rules! foo {
    ($ident:ident) => {
        fn $ident() -> $ident { loop {} }
        struct $ident;
    }
}

foo!(bar$0);
  // ^^^
fn foo() {
    let bar: bar = bar();
          // ^^^
                // ^^^
}
"# ,) ; check (r#"
macro_rules! foo {
    ($ident:ident) => {
        fn $ident() -> $ident { loop {} }
        struct $ident;
    }
}

foo!(bar);
  // ^^^
fn foo() {
    let bar: bar$0 = bar();
          // ^^^
}
"# ,) ; } # [test] fn test_hl_yield_points () { check (r#"
pub async fn foo() {
 // ^^^^^
    let x = foo()
        .await$0
      // ^^^^^
        .await;
      // ^^^^^
    || { 0.await };
    (async { 0.await }).await
                     // ^^^^^
}
"# ,) ; } # [test] fn test_hl_yield_points2 () { check (r#"
pub async$0 fn foo() {
 // ^^^^^
    let x = foo()
        .await
      // ^^^^^
        .await;
      // ^^^^^
    || { 0.await };
    (async { 0.await }).await
                     // ^^^^^
}
"# ,) ; } # [test] fn test_hl_exit_points_of_async_blocks () { check (r#"
pub fn foo() {
    let x = async$0 {
         // ^^^^^
        0.await;
       // ^^^^^
       0?;
     // ^
       return 0;
    // ^^^^^^
       0
    // ^
    };
}
"# ,) ; } # [test] fn test_hl_let_else_yield_points () { check (r#"
pub async fn foo() {
 // ^^^^^
    let x = foo()
        .await$0
      // ^^^^^
        .await;
      // ^^^^^
    || { 0.await };
    let Some(_) = None else {
        foo().await
           // ^^^^^
    };
    (async { 0.await }).await
                     // ^^^^^
}
"# ,) ; } # [test] fn test_hl_yield_nested_fn () { check (r#"
async fn foo() {
    async fn foo2() {
 // ^^^^^
        async fn foo3() {
            0.await
        }
        0.await$0
       // ^^^^^
    }
    0.await
}
"# ,) ; } # [test] fn test_hl_yield_nested_async_blocks () { check (r#"
async fn foo() {
    (async {
  // ^^^^^
        (async { 0.await }).await$0
                         // ^^^^^
    }).await;
}
"# ,) ; } # [test] fn test_hl_exit_points () { check (r#"
  fn foo() -> u32 {
//^^
    if true {
        return$0 0;
     // ^^^^^^
    }

    0?;
  // ^
    0xDEAD_BEEF
 // ^^^^^^^^^^^
}
"# ,) ; } # [test] fn test_hl_exit_points2 () { check (r#"
  fn foo() ->$0 u32 {
//^^
    if true {
        return 0;
     // ^^^^^^
    }

    0?;
  // ^
    0xDEAD_BEEF
 // ^^^^^^^^^^^
}
"# ,) ; } # [test] fn test_hl_exit_points3 () { check (r#"
  fn$0 foo() -> u32 {
//^^
    if true {
        return 0;
     // ^^^^^^
    }

    0?;
  // ^
    0xDEAD_BEEF
 // ^^^^^^^^^^^
}
"# ,) ; } # [test] fn test_hl_let_else_exit_points () { check (r#"
  fn$0 foo() -> u32 {
//^^
    let Some(bar) = None else {
        return 0;
     // ^^^^^^
    };

    0?;
  // ^
    0xDEAD_BEEF
 // ^^^^^^^^^^^
}
"# ,) ; } # [test] fn test_hl_prefer_ref_over_tail_exit () { check (r#"
fn foo() -> u32 {
// ^^^
    if true {
        return 0;
    }

    0?;

    foo$0()
 // ^^^
}
"# ,) ; } # [test] fn test_hl_never_call_is_exit_point () { check (r#"
struct Never;
impl Never {
    fn never(self) -> ! { loop {} }
}
macro_rules! never {
    () => { never() }
         // ^^^^^^^
}
fn never() -> ! { loop {} }
  fn foo() ->$0 u32 {
//^^
    never();
 // ^^^^^^^
    never!();
 // ^^^^^^^^

    Never.never();
 // ^^^^^^^^^^^^^

    0
 // ^
}
"# ,) ; } # [test] fn test_hl_inner_tail_exit_points () { check (r#"
  fn foo() ->$0 u32 {
//^^
    if true {
        unsafe {
            return 5;
         // ^^^^^^
            5
         // ^
        }
    } else if false {
        0
     // ^
    } else {
        match 5 {
            6 => 100,
              // ^^^
            7 => loop {
                break 5;
             // ^^^^^
            }
            8 => 'a: loop {
                'b: loop {
                    break 'a 5;
                 // ^^^^^
                    break 'b 5;
                    break 5;
                };
            }
            //
            _ => 500,
              // ^^^
        }
    }
}
"# ,) ; } # [test] fn test_hl_inner_tail_exit_points_labeled_block () { check (r#"
  fn foo() ->$0 u32 {
//^^
    'foo: {
        break 'foo 0;
     // ^^^^^
        loop {
            break;
            break 'foo 0;
         // ^^^^^
        }
        0
     // ^
    }
}
"# ,) ; } # [test] fn test_hl_inner_tail_exit_points_loops () { check (r#"
  fn foo() ->$0 u32 {
//^^
    'foo: while { return 0; true } {
               // ^^^^^^
        break 'foo 0;
     // ^^^^^
        return 0;
     // ^^^^^^
    }
}
"# ,) ; } # [test] fn test_hl_break_loop () { check (r#"
fn foo() {
    'outer: loop {
 // ^^^^^^^^^^^^
         break;
      // ^^^^^
         'inner: loop {
            break;
            'innermost: loop {
                break 'outer;
             // ^^^^^^^^^^^^
                break 'inner;
            }
            break$0 'outer;
         // ^^^^^^^^^^^^
            break;
        }
        break;
     // ^^^^^
    }
}
"# ,) ; } # [test] fn test_hl_break_loop2 () { check (r#"
fn foo() {
    'outer: loop {
        break;
        'inner: loop {
     // ^^^^^^^^^^^^
            break;
         // ^^^^^
            'innermost: loop {
                break 'outer;
                break 'inner;
             // ^^^^^^^^^^^^
            }
            break 'outer;
            break$0;
         // ^^^^^
        }
        break;
    }
}
"# ,) ; } # [test] fn test_hl_break_for () { check (r#"
fn foo() {
    'outer: for _ in () {
 // ^^^^^^^^^^^
         break;
      // ^^^^^
         'inner: for _ in () {
            break;
            'innermost: for _ in () {
                break 'outer;
             // ^^^^^^^^^^^^
                break 'inner;
            }
            break$0 'outer;
         // ^^^^^^^^^^^^
            break;
        }
        break;
     // ^^^^^
    }
}
"# ,) ; } # [test] fn test_hl_break_for_but_not_continue () { check (r#"
fn foo() {
    'outer: for _ in () {
 // ^^^^^^^^^^^
        break;
     // ^^^^^
        continue;
        'inner: for _ in () {
            break;
            continue;
            'innermost: for _ in () {
                continue 'outer;
                break 'outer;
             // ^^^^^^^^^^^^
                continue 'inner;
                break 'inner;
            }
            break$0 'outer;
         // ^^^^^^^^^^^^
            continue 'outer;
            break;
            continue;
        }
        break;
     // ^^^^^
        continue;
    }
}
"# ,) ; } # [test] fn test_hl_continue_for_but_not_break () { check (r#"
fn foo() {
    'outer: for _ in () {
 // ^^^^^^^^^^^
        break;
        continue;
     // ^^^^^^^^
        'inner: for _ in () {
            break;
            continue;
            'innermost: for _ in () {
                continue 'outer;
             // ^^^^^^^^^^^^^^^
                break 'outer;
                continue 'inner;
                break 'inner;
            }
            break 'outer;
            continue$0 'outer;
         // ^^^^^^^^^^^^^^^
            break;
            continue;
        }
        break;
        continue;
     // ^^^^^^^^
    }
}
"# ,) ; } # [test] fn test_hl_break_and_continue () { check (r#"
fn foo() {
    'outer: fo$0r _ in () {
 // ^^^^^^^^^^^
        break;
     // ^^^^^
        continue;
     // ^^^^^^^^
        'inner: for _ in () {
            break;
            continue;
            'innermost: for _ in () {
                continue 'outer;
             // ^^^^^^^^^^^^^^^
                break 'outer;
             // ^^^^^^^^^^^^
                continue 'inner;
                break 'inner;
            }
            break 'outer;
         // ^^^^^^^^^^^^
            continue 'outer;
         // ^^^^^^^^^^^^^^^
            break;
            continue;
        }
        break;
     // ^^^^^
        continue;
     // ^^^^^^^^
    }
}
"# ,) ; } # [test] fn test_hl_break_while () { check (r#"
fn foo() {
    'outer: while true {
 // ^^^^^^^^^^^^^
         break;
      // ^^^^^
         'inner: while true {
            break;
            'innermost: while true {
                break 'outer;
             // ^^^^^^^^^^^^
                break 'inner;
            }
            break$0 'outer;
         // ^^^^^^^^^^^^
            break;
        }
        break;
     // ^^^^^
    }
}
"# ,) ; } # [test] fn test_hl_break_labeled_block () { check (r#"
fn foo() {
    'outer: {
 // ^^^^^^^
         break;
      // ^^^^^
         'inner: {
            break;
            'innermost: {
                break 'outer;
             // ^^^^^^^^^^^^
                break 'inner;
            }
            break$0 'outer;
         // ^^^^^^^^^^^^
            break;
        }
        break;
     // ^^^^^
    }
}
"# ,) ; } # [test] fn test_hl_break_unlabeled_loop () { check (r#"
fn foo() {
    loop {
 // ^^^^
        break$0;
     // ^^^^^
    }
}
"# ,) ; } # [test] fn test_hl_break_unlabeled_block_in_loop () { check (r#"
fn foo() {
    loop {
 // ^^^^
        {
            break$0;
         // ^^^^^
        }
    }
}
"# ,) ; } # [test] fn test_hl_field_shorthand () { check (r#"
struct Struct { field: u32 }
              //^^^^^
fn function(field: u32) {
          //^^^^^
    Struct { field$0 }
           //^^^^^ read
}
"# ,) ; } # [test] fn test_hl_disabled_ref_local () { let config = HighlightRelatedConfig { references : false , .. ENABLED_CONFIG } ; check_with_config (r#"
fn foo() {
    let x$0 = 5;
    let y = x * 2;
}
"# , config ,) ; } # [test] fn test_hl_disabled_ref_local_preserved_break () { let config = HighlightRelatedConfig { references : false , .. ENABLED_CONFIG } ; check_with_config (r#"
fn foo() {
    let x$0 = 5;
    let y = x * 2;

    loop {
        break;
    }
}
"# , config . clone () ,) ; check_with_config (r#"
fn foo() {
    let x = 5;
    let y = x * 2;

    loop$0 {
//  ^^^^
        break;
//      ^^^^^
    }
}
"# , config ,) ; } # [test] fn test_hl_disabled_ref_local_preserved_yield () { let config = HighlightRelatedConfig { references : false , .. ENABLED_CONFIG } ; check_with_config (r#"
async fn foo() {
    let x$0 = 5;
    let y = x * 2;

    0.await;
}
"# , config . clone () ,) ; check_with_config (r#"
    async fn foo() {
//  ^^^^^
        let x = 5;
        let y = x * 2;

        0.await$0;
//        ^^^^^
}
"# , config ,) ; } # [test] fn test_hl_disabled_ref_local_preserved_exit () { let config = HighlightRelatedConfig { references : false , .. ENABLED_CONFIG } ; check_with_config (r#"
fn foo() -> i32 {
    let x$0 = 5;
    let y = x * 2;

    if true {
        return y;
    }

    0?
}
"# , config . clone () ,) ; check_with_config (r#"
  fn foo() ->$0 i32 {
//^^
    let x = 5;
    let y = x * 2;

    if true {
        return y;
//      ^^^^^^
    }

    0?
//   ^
"# , config ,) ; } # [test] fn test_hl_disabled_break () { let config = HighlightRelatedConfig { break_points : false , .. ENABLED_CONFIG } ; check_with_config (r#"
fn foo() {
    loop {
        break$0;
    }
}
"# , config ,) ; } # [test] fn test_hl_disabled_yield () { let config = HighlightRelatedConfig { yield_points : false , .. ENABLED_CONFIG } ; check_with_config (r#"
async$0 fn foo() {
    0.await;
}
"# , config ,) ; } # [test] fn test_hl_disabled_exit () { let config = HighlightRelatedConfig { exit_points : false , .. ENABLED_CONFIG } ; check_with_config (r#"
fn foo() ->$0 i32 {
    if true {
        return -1;
    }

    42
}"# , config ,) ; } # [test] fn test_hl_multi_local () { check (r#"
fn foo((
    foo$0
  //^^^
    | foo
    //^^^
    | foo
    //^^^
): ()) {
    foo;
  //^^^read
    let foo;
}
"# ,) ; check (r#"
fn foo((
    foo
  //^^^
    | foo$0
    //^^^
    | foo
    //^^^
): ()) {
    foo;
  //^^^read
    let foo;
}
"# ,) ; check (r#"
fn foo((
    foo
  //^^^
    | foo
    //^^^
    | foo
    //^^^
): ()) {
    foo$0;
  //^^^read
    let foo;
}
"# ,) ; } # [test] fn test_hl_trait_impl_methods () { check (r#"
trait Trait {
    fn func$0(self) {}
     //^^^^
}

impl Trait for () {
    fn func(self) {}
     //^^^^
}

fn main() {
    <()>::func(());
        //^^^^
    ().func();
     //^^^^
}
"# ,) ; check (r#"
trait Trait {
    fn func(self) {}
}

impl Trait for () {
    fn func$0(self) {}
     //^^^^
}

fn main() {
    <()>::func(());
        //^^^^
    ().func();
     //^^^^
}
"# ,) ; check (r#"
trait Trait {
    fn func(self) {}
}

impl Trait for () {
    fn func(self) {}
     //^^^^
}

fn main() {
    <()>::func(());
        //^^^^
    ().func$0();
     //^^^^
}
"# ,) ; } # [test] fn test_assoc_type_highlighting () { check (r#"
trait Trait {
    type Output;
      // ^^^^^^
}
impl Trait for () {
    type Output$0 = ();
      // ^^^^^^
}
"# ,) ; } # [test] fn test_closure_capture_pipe () { check (r#"
fn f() {
    let x = 1;
    //  ^
    let c = $0|y| x + y;
    //          ^ read
}
"# ,) ; } # [test] fn test_closure_capture_move () { check (r#"
fn f() {
    let x = 1;
    //  ^
    let c = move$0 |y| x + y;
    //               ^ read
}
"# ,) ; } # [test] fn test_trait_highlights_assoc_item_uses () { check (r#"
trait Super {
    type SuperT;
}
trait Foo: Super {
    //^^^
    type T;
    const C: usize;
    fn f() {}
    fn m(&self) {}
}
impl Foo for i32 {
   //^^^
    type T = i32;
    const C: usize = 0;
    fn f() {}
    fn m(&self) {}
}
fn f<T: Foo$0>(t: T) {
      //^^^
    let _: T::SuperT;
            //^^^^^^
    let _: T::T;
            //^
    t.m();
    //^
    T::C;
     //^
    T::f();
     //^
}

fn f2<T: Foo>(t: T) {
       //^^^
    let _: T::T;
    t.m();
    T::C;
    T::f();
}
"# ,) ; } # [test] fn test_trait_highlights_assoc_item_uses_use_tree () { check (r#"
use Foo$0;
 // ^^^ import
trait Super {
    type SuperT;
}
trait Foo: Super {
    //^^^
    type T;
    const C: usize;
    fn f() {}
    fn m(&self) {}
}
impl Foo for i32 {
   //^^^
    type T = i32;
      // ^
    const C: usize = 0;
       // ^
    fn f() {}
    // ^
    fn m(&self) {}
    // ^
}
fn f<T: Foo>(t: T) {
      //^^^
    let _: T::SuperT;
    let _: T::T;
            //^
    t.m();
    //^
    T::C;
     //^
    T::f();
     //^
}
"# ,) ; } # [test] fn implicit_format_args () { check (r#"
//- minicore: fmt
fn test() {
    let a = "foo";
     // ^
    format_args!("hello {a} {a$0} {}", a);
                      // ^read
                          // ^read
                                  // ^read
}
"# ,) ; } # [test] fn return_in_macros () { check (r#"
macro_rules! N {
    ($i:ident, $x:expr, $blk:expr) => {
        for $i in 0..$x {
            $blk
        }
    };
}

fn main() {
    fn f() {
 // ^^
        N!(i, 5, {
            println!("{}", i);
            return$0;
         // ^^^^^^
        });

        for i in 1..5 {
            return;
         // ^^^^^^
        }
       (|| {
            return;
        })();
    }
}
"# ,) } # [test] fn return_in_closure () { check (r#"
macro_rules! N {
    ($i:ident, $x:expr, $blk:expr) => {
        for $i in 0..$x {
            $blk
        }
    };
}

fn main() {
    fn f() {
        N!(i, 5, {
            println!("{}", i);
            return;
        });

        for i in 1..5 {
            return;
        }
       (|| {
     // ^
            return$0;
         // ^^^^^^
        })();
    }
}
"# ,) } # [test] fn return_in_try () { check (r#"
fn main() {
    fn f() {
 // ^^
        try {
            return$0;
         // ^^^^^^
        }

        return;
     // ^^^^^^
    }
}
"# ,) } # [test] fn break_in_try () { check (r#"
fn main() {
    for i in 1..100 {
 // ^^^
        let x: Result<(), ()> = try {
            break$0;
         // ^^^^^
        };
    }
}
"# ,) } # [test] fn no_highlight_on_return_in_macro_call () { check (r#"
//- minicore:include
//- /lib.rs
macro_rules! M {
    ($blk:expr) => {
        $blk
    };
}

fn main() {
    fn f() {
 // ^^
        M!({ return$0; });
          // ^^^^^^
     // ^^^^^^^^^^^^^^^

        include!("a.rs")
     // ^^^^^^^^^^^^^^^^
    }
}

//- /a.rs
{
    return;
}
"# ,) } # [test] fn nested_match () { check (r#"
fn main() {
    match$0 0 {
 // ^^^^^
        0 => match 1 {
            1 => 2,
              // ^
            _ => 3,
              // ^
        },
        _ => 4,
          // ^
    }
}
"# ,) } # [test] fn single_arm_highlight () { check (r#"
fn main() {
    match 0 {
        0 =>$0 {
       // ^^
            let x = 1;
            x
         // ^
        }
        _ => 2,
    }
}
"# ,) } # [test] fn no_branches_when_disabled () { let config = HighlightRelatedConfig { branch_exit_points : false , .. ENABLED_CONFIG } ; check_with_config (r#"
fn main() {
    match$0 0 {
        0 => 1,
        _ => 2,
    }
}
"# , config ,) ; } # [test] fn asm () { check (r#"
//- minicore: asm
#[inline]
pub unsafe fn bootstrap() -> ! {
    builtin#asm(
        "blabla",
        "mrs {tmp}, CONTROL",
           // ^^^ read
        "blabla",
        "bics {tmp}, {spsel}",
            // ^^^ read
        "blabla",
        "msr CONTROL, {tmp}",
                    // ^^^ read
        "blabla",
        tmp$0 = inout(reg) 0,
     // ^^^
        aaa = in(reg) 2,
        aaa = in(reg) msp,
        aaa = in(reg) rv,
        options(noreturn, nomem, nostack),
    );
}
"# ,) } # [test] fn complex_arms_highlight () { check (r#"
fn calculate(n: i32) -> i32 { n * 2 }

fn main() {
    match$0 Some(1) {
 // ^^^^^
        Some(x) => match x {
            0 => { let y = x; y },
                           // ^
            1 => calculate(x),
               //^^^^^^^^^^^^
            _ => (|| 6)(),
              // ^^^^^^^^
        },
        None => loop {
            break 5;
         // ^^^^^^^
        },
    }
}
"# ,) } # [test] fn match_in_macro_highlight () { check (r#"
macro_rules! M {
    ($e:expr) => { $e };
}

fn main() {
    M!{
        match$0 Some(1) {
     // ^^^^^
            Some(x) => x,
                    // ^
            None => 0,
                 // ^
        }
    }
}
"# ,) } # [test] fn match_in_macro_highlight_2 () { check (r#"
macro_rules! match_ast {
    (match $node:ident { $($tt:tt)* }) => { $crate::match_ast!(match ($node) { $($tt)* }) };

    (match ($node:expr) {
        $( $( $path:ident )::+ ($it:pat) => $res:expr, )*
        _ => $catch_all:expr $(,)?
    }) => {{
        $( if let Some($it) = $($path::)+cast($node.clone()) { $res } else )*
        { $catch_all }
    }};
}

fn main() {
    match_ast! {
        match$0 Some(1) {
            Some(x) => x,
        }
    }
}
            "# ,) ; } # [test] fn nested_if_else () { check (r#"
fn main() {
    if$0 true {
 // ^^
        if false {
            1
         // ^
        } else {
            2
         // ^
        }
    } else {
        3
     // ^
    }
}
"# ,) } # [test] fn if_else_if_highlight () { check (r#"
fn main() {
    if$0 true {
 // ^^
        1
     // ^
    } else if false {
        // ^^
        2
     // ^
    } else {
        3
     // ^
    }
}
"# ,) } # [test] fn complex_if_branches () { check (r#"
fn calculate(n: i32) -> i32 { n * 2 }

fn main() {
    if$0 true {
 // ^^
        let x = 5;
        calculate(x)
     // ^^^^^^^^^^^^
    } else if false {
        // ^^
        (|| 10)()
     // ^^^^^^^^^
    } else {
        loop {
            break 15;
         // ^^^^^^^^
        }
    }
}
"# ,) } # [test] fn if_in_macro_highlight () { check (r#"
macro_rules! M {
    ($e:expr) => { $e };
}

fn main() {
    M!{
        if$0 true {
     // ^^
            5
         // ^
        } else {
            10
         // ^^
        }
    }
}
"# ,) } # [test] fn match_in_macro () { check (r#"
macro_rules! M {
    (match) => { 1 };
}

fn main() {
    match Some(1) {
        Some(x) => x,
        None => {
            M!(match$0)
        }
    }
}
            "# ,) } # [test] fn labeled_block_tail_expr () { check (r#"
fn foo() {
    'a: {
 // ^^^
        if true { break$0 'a 0; }
               // ^^^^^^^^
        5
     // ^
    }
}
"# ,) ; } # [test] fn labeled_block_tail_expr_2 () { check (r#"
fn foo() {
    let _ = 'b$0lk: {
         // ^^^^
        let x = 1;
        if true { break 'blk 42; }
                     // ^^^^
        if false { break 'blk 24; }
                      // ^^^^
        100
     // ^^^
    };
}
"# ,) ; } # [test] fn different_unsafe_block () { check (r#"
fn main() {
    unsafe$0 {
 // ^^^^^^
        *(0 as *const u8)
     // ^^^^^^^^^^^^^^^^^
    };
    unsafe { *(1 as *const u8) };
    unsafe { *(2 as *const u8) };
}
        "# ,) ; } }
};
}

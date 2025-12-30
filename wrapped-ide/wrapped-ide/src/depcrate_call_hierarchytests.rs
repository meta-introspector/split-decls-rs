// Generated macro for tests (module)
macro_rules! Depcrate_call_hierarchytests {
() => {
// Module: crate::call_hierarchy
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use expect_test :: { Expect , expect } ; use ide_db :: { FilePosition , MiniCore } ; use itertools :: Itertools ; use crate :: fixture ; fn check_hierarchy (exclude_tests : bool , # [rust_analyzer :: rust_fixture] ra_fixture : & str , expected_nav : Expect , expected_incoming : Expect , expected_outgoing : Expect ,) { fn debug_render (item : crate :: CallItem) -> String { format ! ("{} : {}" , item . target . debug_render () , item . ranges . iter () . format_with (", " , | range , f | f (& format_args ! ("{:?}:{:?}" , range . file_id , range . range)))) } let config = crate :: CallHierarchyConfig { exclude_tests , minicore : MiniCore :: default () } ; let (analysis , pos) = fixture :: position (ra_fixture) ; let mut navs = analysis . call_hierarchy (pos , & config) . unwrap () . unwrap () . info ; assert_eq ! (navs . len () , 1) ; let nav = navs . pop () . unwrap () ; expected_nav . assert_eq (& nav . debug_render ()) ; let item_pos = FilePosition { file_id : nav . file_id , offset : nav . focus_or_full_range () . start () } ; let incoming_calls = analysis . incoming_calls (& config , item_pos) . unwrap () . unwrap () ; expected_incoming . assert_eq (& incoming_calls . into_iter () . map (debug_render) . join ("\n")) ; let outgoing_calls = analysis . outgoing_calls (& config , item_pos) . unwrap () . unwrap () ; expected_outgoing . assert_eq (& outgoing_calls . into_iter () . map (debug_render) . join ("\n")) ; } # [test] fn test_call_hierarchy_on_ref () { check_hierarchy (false , r#"
//- /lib.rs
fn callee() {}
fn caller() {
    call$0ee();
}
"# , expect ! [["callee Function FileId(0) 0..14 3..9"]] , expect ! ["caller Function FileId(0) 15..44 18..24 : FileId(0):33..39"] , expect ! [[]] ,) ; } # [test] fn test_call_hierarchy_on_def () { check_hierarchy (false , r#"
//- /lib.rs
fn call$0ee() {}
fn caller() {
    callee();
}
"# , expect ! [["callee Function FileId(0) 0..14 3..9"]] , expect ! ["caller Function FileId(0) 15..44 18..24 : FileId(0):33..39"] , expect ! [[]] ,) ; } # [test] fn test_call_hierarchy_in_same_fn () { check_hierarchy (false , r#"
//- /lib.rs
fn callee() {}
fn caller() {
    call$0ee();
    callee();
}
"# , expect ! [["callee Function FileId(0) 0..14 3..9"]] , expect ! ["caller Function FileId(0) 15..58 18..24 : FileId(0):33..39, FileId(0):47..53"] , expect ! [[]] ,) ; } # [test] fn test_call_hierarchy_in_different_fn () { check_hierarchy (false , r#"
//- /lib.rs
fn callee() {}
fn caller1() {
    call$0ee();
}

fn caller2() {
    callee();
}
"# , expect ! [["callee Function FileId(0) 0..14 3..9"]] , expect ! [[r#"
                caller1 Function FileId(0) 15..45 18..25 : FileId(0):34..40
                caller2 Function FileId(0) 47..77 50..57 : FileId(0):66..72"#]] , expect ! [[]] ,) ; } # [test] fn test_call_hierarchy_in_tests_mod () { check_hierarchy (false , r#"
//- /lib.rs cfg:test
fn callee() {}
fn caller1() {
    call$0ee();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_caller() {
        callee();
    }
}
"# , expect ! [["callee Function FileId(0) 0..14 3..9"]] , expect ! [[r#"
                caller1 Function FileId(0) 15..45 18..25 : FileId(0):34..40
                test_caller Function FileId(0) 95..149 110..121 tests : FileId(0):134..140"#]] , expect ! [[]] ,) ; } # [test] fn test_call_hierarchy_in_different_files () { check_hierarchy (false , r#"
//- /lib.rs
mod foo;
use foo::callee;

fn caller() {
    call$0ee();
}

//- /foo/mod.rs
pub fn callee() {}
"# , expect ! ["callee Function FileId(1) 0..18 7..13 foo"] , expect ! ["caller Function FileId(0) 27..56 30..36 : FileId(0):45..51"] , expect ! [[]] ,) ; } # [test] fn test_call_hierarchy_outgoing () { check_hierarchy (false , r#"
//- /lib.rs
fn callee() {}
fn call$0er() {
    callee();
    callee();
}
"# , expect ! [["caller Function FileId(0) 15..58 18..24"]] , expect ! [[]] , expect ! ["callee Function FileId(0) 0..14 3..9 : FileId(0):33..39, FileId(0):47..53"] ,) ; } # [test] fn test_call_hierarchy_outgoing_in_different_files () { check_hierarchy (false , r#"
//- /lib.rs
mod foo;
use foo::callee;

fn call$0er() {
    callee();
}

//- /foo/mod.rs
pub fn callee() {}
"# , expect ! [["caller Function FileId(0) 27..56 30..36"]] , expect ! [[]] , expect ! ["callee Function FileId(1) 0..18 7..13 foo : FileId(0):45..51"] ,) ; } # [test] fn test_call_hierarchy_incoming_outgoing () { check_hierarchy (false , r#"
//- /lib.rs
fn caller1() {
    call$0er2();
}

fn caller2() {
    caller3();
}

fn caller3() {

}
"# , expect ! [["caller2 Function FileId(0) 33..64 36..43"]] , expect ! ["caller1 Function FileId(0) 0..31 3..10 : FileId(0):19..26"] , expect ! ["caller3 Function FileId(0) 66..83 69..76 : FileId(0):52..59"] ,) ; } # [test] fn test_call_hierarchy_issue_5103 () { check_hierarchy (false , r#"
fn a() {
    b()
}

fn b() {}

fn main() {
    a$0()
}
"# , expect ! [["a Function FileId(0) 0..18 3..4"]] , expect ! ["main Function FileId(0) 31..52 34..38 : FileId(0):47..48"] , expect ! ["b Function FileId(0) 20..29 23..24 : FileId(0):13..14"] ,) ; check_hierarchy (false , r#"
fn a() {
    b$0()
}

fn b() {}

fn main() {
    a()
}
"# , expect ! [["b Function FileId(0) 20..29 23..24"]] , expect ! ["a Function FileId(0) 0..18 3..4 : FileId(0):13..14"] , expect ! [[]] ,) ; } # [test] fn test_call_hierarchy_in_macros_incoming () { check_hierarchy (false , r#"
macro_rules! define {
    ($ident:ident) => {
        fn $ident {}
    }
}
macro_rules! call {
    ($ident:ident) => {
        $ident()
    }
}
define!(callee)
fn caller() {
    call!(call$0ee);
}
"# , expect ! [[r#"callee Function FileId(0) 144..159 152..158"#]] , expect ! ["caller Function FileId(0) 160..194 163..169 : FileId(0):184..190"] , expect ! [[]] ,) ; check_hierarchy (false , r#"
macro_rules! define {
    ($ident:ident) => {
        fn $ident {}
    }
}
macro_rules! call {
    ($ident:ident) => {
        $ident()
    }
}
define!(cal$0lee)
fn caller() {
    call!(callee);
}
"# , expect ! [[r#"callee Function FileId(0) 144..159 152..158"#]] , expect ! ["caller Function FileId(0) 160..194 163..169 : FileId(0):184..190"] , expect ! [[]] ,) ; } # [test] fn test_call_hierarchy_in_macros_outgoing () { check_hierarchy (false , r#"
macro_rules! define {
    ($ident:ident) => {
        fn $ident {}
    }
}
macro_rules! call {
    ($ident:ident) => {
        $ident()
    }
}
define!(callee)
fn caller$0() {
    call!(callee);
}
"# , expect ! [[r#"caller Function FileId(0) 160..194 163..169"#]] , expect ! [[]] , expect ! [[]] ,) ; } # [test] fn test_call_hierarchy_in_macros_incoming_different_files () { check_hierarchy (false , r#"
//- /lib.rs
#[macro_use]
mod foo;
define!(callee)
fn caller() {
    call!(call$0ee);
}
//- /foo.rs
macro_rules! define {
    ($ident:ident) => {
        fn $ident {}
    }
}
macro_rules! call {
    ($ident:ident) => {
        $ident()
    }
}
"# , expect ! ["callee Function FileId(0) 22..37 30..36"] , expect ! ["caller Function FileId(0) 38..72 41..47 : FileId(0):62..68"] , expect ! [[]] ,) ; check_hierarchy (false , r#"
//- /lib.rs
#[macro_use]
mod foo;
define!(cal$0lee)
fn caller() {
    call!(callee);
}
//- /foo.rs
macro_rules! define {
    ($ident:ident) => {
        fn $ident {}
    }
}
macro_rules! call {
    ($ident:ident) => {
        $ident()
    }
}
"# , expect ! ["callee Function FileId(0) 22..37 30..36"] , expect ! ["caller Function FileId(0) 38..72 41..47 : FileId(0):62..68"] , expect ! [[]] ,) ; check_hierarchy (false , r#"
//- /lib.rs
#[macro_use]
mod foo;
define!(cal$0lee)
call!(callee);
//- /foo.rs
macro_rules! define {
    ($ident:ident) => {
        fn $ident {}
    }
}
macro_rules! call {
    ($ident:ident) => {
        fn caller() {
            $ident()
        }
        fn $ident() {
            $ident()
        }
    }
}
"# , expect ! ["callee Function FileId(0) 22..37 30..36"] , expect ! [[r#"
                caller Function FileId(0) 38..43 : FileId(0):44..50
                caller Function FileId(1) 130..136 130..136 : FileId(0):44..50
                callee Function FileId(0) 38..52 44..50 : FileId(0):44..50"#]] , expect ! [[]] ,) ; } # [test] fn test_call_hierarchy_in_macros_outgoing_different_files () { check_hierarchy (false , r#"
//- /lib.rs
#[macro_use]
mod foo;
define!(callee)
fn caller$0() {
    call!(callee);
}
//- /foo.rs
macro_rules! define {
    ($ident:ident) => {
        fn $ident {}
    }
}
macro_rules! call {
    ($ident:ident) => {
        $ident()
        callee()
    }
}
"# , expect ! ["caller Function FileId(0) 38..72 41..47"] , expect ! [[]] , expect ! [[]] ,) ; check_hierarchy (false , r#"
//- /lib.rs
#[macro_use]
mod foo;
define!(callee)
fn caller$0() {
    call!(callee);
}
//- /foo.rs
macro_rules! define {
    () => {
        fn callee {}
    }
}
macro_rules! call {
    ($ident:ident) => {
        $ident()
        callee()
    }
}
"# , expect ! ["caller Function FileId(0) 38..72 41..47"] , expect ! [[]] , expect ! [[]] ,) ; } # [test] fn test_trait_method_call_hierarchy () { check_hierarchy (false , r#"
trait T1 {
    fn call$0ee();
}

struct S1;

impl T1 for S1 {
    fn callee() {}
}

fn caller() {
    S1::callee();
}
"# , expect ! ["callee Function FileId(0) 15..27 18..24 T1"] , expect ! ["caller Function FileId(0) 82..115 85..91 : FileId(0):104..110"] , expect ! [[]] ,) ; } # [test] fn test_call_hierarchy_excluding_tests () { check_hierarchy (false , r#"
fn main() {
    f1();
}

fn f1$0() {
    f2(); f3();
}

fn f2() {
    f1(); f3();
}

#[test]
fn f3() {
    f1(); f2();
}
"# , expect ! ["f1 Function FileId(0) 25..52 28..30"] , expect ! [[r#"
                main Function FileId(0) 0..23 3..7 : FileId(0):16..18
                f2 Function FileId(0) 54..81 57..59 : FileId(0):68..70
                f3 Function FileId(0) 83..118 94..96 : FileId(0):105..107"#]] , expect ! [[r#"
                f2 Function FileId(0) 54..81 57..59 : FileId(0):39..41
                f3 Function FileId(0) 83..118 94..96 : FileId(0):45..47"#]] ,) ; check_hierarchy (true , r#"
fn main() {
    f1();
}

fn f1$0() {
    f2(); f3();
}

fn f2() {
    f1(); f3();
}

#[test]
fn f3() {
    f1(); f2();
}
"# , expect ! ["f1 Function FileId(0) 25..52 28..30"] , expect ! [[r#"
                main Function FileId(0) 0..23 3..7 : FileId(0):16..18
                f2 Function FileId(0) 54..81 57..59 : FileId(0):68..70"#]] , expect ! ["f2 Function FileId(0) 54..81 57..59 : FileId(0):39..41"] ,) ; } }
};
}

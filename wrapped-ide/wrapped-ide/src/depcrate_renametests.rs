// Generated macro for tests (module)
macro_rules! Depcrate_renametests {
() => {
// Module: crate::rename
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use expect_test :: { Expect , expect } ; use ide_db :: source_change :: SourceChange ; use ide_db :: text_edit :: TextEdit ; use itertools :: Itertools ; use stdx :: trim_indent ; use test_utils :: assert_eq_text ; use crate :: fixture ; use super :: { RangeInfo , RenameConfig , RenameError } ; const TEST_CONFIG : RenameConfig = RenameConfig { prefer_no_std : false , prefer_prelude : true , prefer_absolute : false } ; # [track_caller] fn check (new_name : & str , # [rust_analyzer :: rust_fixture] ra_fixture_before : & str , # [rust_analyzer :: rust_fixture] ra_fixture_after : & str ,) { let ra_fixture_after = & trim_indent (ra_fixture_after) ; let (analysis , position) = fixture :: position (ra_fixture_before) ; if ! ra_fixture_after . starts_with ("error: ") && let Err (err) = analysis . prepare_rename (position) . unwrap () { panic ! ("Prepare rename to '{new_name}' was failed: {err}") } let rename_result = analysis . rename (position , new_name , & TEST_CONFIG) . unwrap_or_else (| err | panic ! ("Rename to '{new_name}' was cancelled: {err}")) ; match rename_result { Ok (source_change) => { let mut text_edit_builder = TextEdit :: builder () ; let (& file_id , edit) = match source_change . source_file_edits . len () { 0 => return , 1 => source_change . source_file_edits . iter () . next () . unwrap () , _ => panic ! () , } ; for indel in edit . 0 . iter () { text_edit_builder . replace (indel . delete , indel . insert . clone ()) ; } let mut result = analysis . file_text (file_id) . unwrap () . to_string () ; text_edit_builder . finish () . apply (& mut result) ; assert_eq_text ! (ra_fixture_after , &* result) ; } Err (err) => { if ra_fixture_after . starts_with ("error:") { let error_message = ra_fixture_after . chars () . skip ("error:" . len ()) . collect :: < String > () ; assert_eq ! (error_message . trim () , err . to_string ()) ; } else { panic ! ("Rename to '{new_name}' failed unexpectedly: {err}") } } } ; } # [track_caller] fn check_conflicts (new_name : & str , # [rust_analyzer :: rust_fixture] ra_fixture : & str) { let (analysis , position , conflicts) = fixture :: annotations (ra_fixture) ; let source_change = analysis . rename (position , new_name , & TEST_CONFIG) . unwrap () . unwrap () ; let expected_conflicts = conflicts . into_iter () . map (| (file_range , _) | (file_range . file_id , file_range . range)) . sorted_unstable_by_key (| (file_id , range) | (* file_id , range . start ())) . collect_vec () ; let found_conflicts = source_change . source_file_edits . iter () . filter (| (_ , (edit , _)) | edit . change_annotation () . is_some ()) . flat_map (| (file_id , (edit , _)) | { edit . into_iter () . map (move | edit | (* file_id , edit . delete)) }) . sorted_unstable_by_key (| (file_id , range) | (* file_id , range . start ())) . collect_vec () ; assert_eq ! (expected_conflicts , found_conflicts , "rename conflicts mismatch: {source_change:#?}") ; } fn check_expect (new_name : & str , # [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect ,) { let (analysis , position) = fixture :: position (ra_fixture) ; let source_change = analysis . rename (position , new_name , & TEST_CONFIG) . unwrap () . expect ("Expect returned a RenameError") ; expect . assert_eq (& filter_expect (source_change)) } fn check_expect_will_rename_file (new_name : & str , # [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect ,) { let (analysis , position) = fixture :: position (ra_fixture) ; let source_change = analysis . will_rename_file (position . file_id , new_name) . unwrap () . expect ("Expect returned a RenameError") ; expect . assert_eq (& filter_expect (source_change)) } fn check_prepare (# [rust_analyzer :: rust_fixture] ra_fixture : & str , expect : Expect) { let (analysis , position) = fixture :: position (ra_fixture) ; let result = analysis . prepare_rename (position) . unwrap_or_else (| err | panic ! ("PrepareRename was cancelled: {err}")) ; match result { Ok (RangeInfo { range , info : () }) => { let source = analysis . file_text (position . file_id) . unwrap () ; expect . assert_eq (& format ! ("{range:?}: {}" , & source [range])) } Err (RenameError (err)) => expect . assert_eq (& err) , } ; } fn filter_expect (source_change : SourceChange) -> String { let source_file_edits = source_change . source_file_edits . into_iter () . map (| (id , (text_edit , _)) | (id , text_edit . into_iter () . collect :: < Vec < _ > > ())) . collect :: < Vec < _ > > () ; format ! ("source_file_edits: {:#?}\nfile_system_edits: {:#?}\n" , source_file_edits , source_change . file_system_edits) } # [test] fn rename_will_shadow () { check_conflicts ("new_name" , r#"
fn foo() {
    let mut new_name = 123;
    let old_name$0 = 456;
     // ^^^^^^^^
    new_name = 789 + new_name;
}
        "# ,) ; } # [test] fn rename_will_be_shadowed () { check_conflicts ("new_name" , r#"
fn foo() {
    let mut old_name$0 = 456;
         // ^^^^^^^^
    let new_name = 123;
    old_name = 789 + old_name;
 // ^^^^^^^^         ^^^^^^^^
}
        "# ,) ; } # [test] fn test_prepare_rename_namelikes () { check_prepare (r"fn name$0<'lifetime>() {}" , expect ! [[r#"3..7: name"#]]) ; check_prepare (r"fn name<'lifetime$0>() {}" , expect ! [[r#"9..17: lifetime"#]]) ; check_prepare (r"fn name<'lifetime>() { name$0(); }" , expect ! [[r#"23..27: name"#]]) ; } # [test] fn test_prepare_rename_in_macro () { check_prepare (r"macro_rules! foo {
    ($ident:ident) => {
        pub struct $ident;
    }
}
foo!(Foo$0);" , expect ! [[r#"83..86: Foo"#]] ,) ; } # [test] fn test_prepare_rename_keyword () { check_prepare (r"struct$0 Foo;" , expect ! [[r#"No references found at position"#]]) ; } # [test] fn test_prepare_rename_tuple_field () { check_prepare (r#"
struct Foo(i32);

fn baz() {
    let mut x = Foo(4);
    x.0$0 = 5;
}
"# , expect ! [[r#"No references found at position"#]] ,) ; } # [test] fn test_prepare_rename_builtin () { check_prepare (r#"
fn foo() {
    let x: i32$0 = 0;
}
"# , expect ! [[r#"No references found at position"#]] ,) ; } # [test] fn test_prepare_rename_self () { check_prepare (r#"
struct Foo {}

impl Foo {
    fn foo(self) -> Self$0 {
        self
    }
}
"# , expect ! [[r#"No references found at position"#]] ,) ; } # [test] fn test_rename_to_underscore () { check ("_" , r#"fn main() { let i$0 = 1; }"# , r#"fn main() { let _ = 1; }"#) ; } # [test] fn test_rename_to_raw_identifier () { check ("r#fn" , r#"fn main() { let i$0 = 1; }"# , r#"fn main() { let r#fn = 1; }"#) ; } # [test] fn test_rename_to_invalid_identifier1 () { check ("invalid!" , r#"fn main() { let i$0 = 1; }"# , "error: Invalid name `invalid!`: not an identifier" ,) ; } # [test] fn test_rename_to_invalid_identifier2 () { check ("multiple tokens" , r#"fn main() { let i$0 = 1; }"# , "error: Invalid name `multiple tokens`: not an identifier" ,) ; } # [test] fn test_rename_to_invalid_identifier3 () { check ("super" , r#"fn main() { let i$0 = 1; }"# , "error: Invalid name `super`: cannot rename to a keyword" ,) ; } # [test] fn test_rename_to_invalid_identifier_lifetime () { cov_mark :: check ! (rename_not_an_ident_ref) ; check ("'foo" , r#"fn main() { let i$0 = 1; }"# , "error: Invalid name `'foo`: not an identifier" ,) ; } # [test] fn test_rename_to_invalid_identifier_lifetime2 () { check ("_" , r#"fn main<'a>(_: &'a$0 ()) {}"# , r#"error: Invalid name `_`: not a lifetime identifier"# ,) ; } # [test] fn test_rename_accepts_lifetime_without_apostrophe () { check ("foo" , r#"fn main<'a>(_: &'a$0 ()) {}"# , r#"fn main<'foo>(_: &'foo ()) {}"#) ; } # [test] fn test_rename_to_underscore_invalid () { cov_mark :: check ! (rename_underscore_multiple) ; check ("_" , r#"fn main(foo$0: ()) {foo;}"# , "error: Cannot rename reference to `_` as it is being referenced multiple times" ,) ; } # [test] fn test_rename_mod_invalid () { check ("'foo" , r#"mod foo$0 {}"# , "error: Invalid name `'foo`: cannot rename module to 'foo" ,) ; } # [test] fn test_rename_mod_invalid_raw_ident () { check ("r#self" , r#"mod foo$0 {}"# , "error: Invalid name `self`: cannot rename module to self" ,) ; } # [test] fn test_rename_for_local () { check ("k" , r#"
fn main() {
    let mut i = 1;
    let j = 1;
    i = i$0 + j;

    { i = 0; }

    i = 5;
}
"# , r#"
fn main() {
    let mut k = 1;
    let j = 1;
    k = k + j;

    { k = 0; }

    k = 5;
}
"# ,) ; } # [test] fn test_rename_unresolved_reference () { check ("new_name" , r#"fn main() { let _ = unresolved_ref$0; }"# , "error: No references found at position" ,) ; } # [test] fn test_rename_macro_multiple_occurrences () { check ("Baaah" , r#"macro_rules! foo {
    ($ident:ident) => {
        const $ident: () = ();
        struct $ident {}
    };
}

foo!($0Foo);
const _: () = Foo;
const _: Foo = Foo {};
    "# , r#"
macro_rules! foo {
    ($ident:ident) => {
        const $ident: () = ();
        struct $ident {}
    };
}

foo!(Baaah);
const _: () = Baaah;
const _: Baaah = Baaah {};
    "# ,) } # [test] fn test_rename_for_macro_args () { check ("b" , r#"
macro_rules! foo {($i:ident) => {$i} }
fn main() {
    let a$0 = "test";
    foo!(a);
}
"# , r#"
macro_rules! foo {($i:ident) => {$i} }
fn main() {
    let b = "test";
    foo!(b);
}
"# ,) ; } # [test] fn test_rename_for_macro_args_rev () { check ("b" , r#"
macro_rules! foo {($i:ident) => {$i} }
fn main() {
    let a = "test";
    foo!(a$0);
}
"# , r#"
macro_rules! foo {($i:ident) => {$i} }
fn main() {
    let b = "test";
    foo!(b);
}
"# ,) ; } # [test] fn test_rename_for_macro_define_fn () { check ("bar" , r#"
macro_rules! define_fn {($id:ident) => { fn $id{} }}
define_fn!(foo);
fn main() {
    fo$0o();
}
"# , r#"
macro_rules! define_fn {($id:ident) => { fn $id{} }}
define_fn!(bar);
fn main() {
    bar();
}
"# ,) ; } # [test] fn test_rename_for_macro_define_fn_rev () { check ("bar" , r#"
macro_rules! define_fn {($id:ident) => { fn $id{} }}
define_fn!(fo$0o);
fn main() {
    foo();
}
"# , r#"
macro_rules! define_fn {($id:ident) => { fn $id{} }}
define_fn!(bar);
fn main() {
    bar();
}
"# ,) ; } # [test] fn test_rename_for_param_inside () { check ("j" , r#"fn foo(i : u32) -> u32 { i$0 }"# , r#"fn foo(j : u32) -> u32 { j }"#) ; } # [test] fn test_rename_refs_for_fn_param () { check ("j" , r#"fn foo(i$0 : u32) -> u32 { i }"# , r#"fn foo(j : u32) -> u32 { j }"#) ; } # [test] fn test_rename_for_mut_param () { check ("j" , r#"fn foo(mut i$0 : u32) -> u32 { i }"# , r#"fn foo(mut j : u32) -> u32 { j }"#) ; } # [test] fn test_rename_struct_field () { check ("foo" , r#"
struct Foo { field$0: i32 }

impl Foo {
    fn new(i: i32) -> Self {
        Self { field: i }
    }
}
"# , r#"
struct Foo { foo: i32 }

impl Foo {
    fn new(i: i32) -> Self {
        Self { foo: i }
    }
}
"# ,) ; } # [test] fn test_rename_field_in_field_shorthand () { cov_mark :: check ! (test_rename_field_in_field_shorthand) ; check ("field" , r#"
struct Foo { foo$0: i32 }

impl Foo {
    fn new(foo: i32) -> Self {
        Self { foo }
    }
}
"# , r#"
struct Foo { field: i32 }

impl Foo {
    fn new(foo: i32) -> Self {
        Self { field: foo }
    }
}
"# ,) ; } # [test] fn test_rename_local_in_field_shorthand () { cov_mark :: check ! (test_rename_local_in_field_shorthand) ; check ("j" , r#"
struct Foo { i: i32 }

impl Foo {
    fn new(i$0: i32) -> Self {
        Self { i }
    }
}
"# , r#"
struct Foo { i: i32 }

impl Foo {
    fn new(j: i32) -> Self {
        Self { i: j }
    }
}
"# ,) ; } # [test] fn test_field_shorthand_correct_struct () { check ("j" , r#"
struct Foo { i$0: i32 }
struct Bar { i: i32 }

impl Bar {
    fn new(i: i32) -> Self {
        Self { i }
    }
}
"# , r#"
struct Foo { j: i32 }
struct Bar { i: i32 }

impl Bar {
    fn new(i: i32) -> Self {
        Self { i }
    }
}
"# ,) ; } # [test] fn test_shadow_local_for_struct_shorthand () { check ("j" , r#"
struct Foo { i: i32 }

fn baz(i$0: i32) -> Self {
     let x = Foo { i };
     {
         let i = 0;
         Foo { i }
     }
}
"# , r#"
struct Foo { i: i32 }

fn baz(j: i32) -> Self {
     let x = Foo { i: j };
     {
         let i = 0;
         Foo { i }
     }
}
"# ,) ; } # [test] fn test_rename_mod () { check_expect ("foo2" , r#"
//- /lib.rs
mod bar;

//- /bar.rs
mod foo$0;

//- /bar/foo.rs
// empty
"# , expect ! [[r#"
                source_file_edits: [
                    (
                        FileId(
                            1,
                        ),
                        [
                            Indel {
                                insert: "foo2",
                                delete: 4..7,
                            },
                        ],
                    ),
                ]
                file_system_edits: [
                    MoveFile {
                        src: FileId(
                            2,
                        ),
                        dst: AnchoredPathBuf {
                            anchor: FileId(
                                2,
                            ),
                            path: "foo2.rs",
                        },
                    },
                ]
            "#]] ,) ; } # [test] fn test_rename_mod_in_use_tree () { check_expect ("quux" , r#"
//- /main.rs
pub mod foo;
pub mod bar;
fn main() {}

//- /foo.rs
pub struct FooContent;

//- /bar.rs
use crate::foo$0::FooContent;
"# , expect ! [[r#"
                source_file_edits: [
                    (
                        FileId(
                            0,
                        ),
                        [
                            Indel {
                                insert: "quux",
                                delete: 8..11,
                            },
                        ],
                    ),
                    (
                        FileId(
                            2,
                        ),
                        [
                            Indel {
                                insert: "quux",
                                delete: 11..14,
                            },
                        ],
                    ),
                ]
                file_system_edits: [
                    MoveFile {
                        src: FileId(
                            1,
                        ),
                        dst: AnchoredPathBuf {
                            anchor: FileId(
                                1,
                            ),
                            path: "quux.rs",
                        },
                    },
                ]
            "#]] ,) ; } # [test] fn test_rename_mod_in_dir () { check_expect ("foo2" , r#"
//- /lib.rs
mod fo$0o;
//- /foo/mod.rs
// empty
"# , expect ! [[r#"
                source_file_edits: [
                    (
                        FileId(
                            0,
                        ),
                        [
                            Indel {
                                insert: "foo2",
                                delete: 4..7,
                            },
                        ],
                    ),
                ]
                file_system_edits: [
                    MoveDir {
                        src: AnchoredPathBuf {
                            anchor: FileId(
                                1,
                            ),
                            path: "../foo",
                        },
                        src_id: FileId(
                            1,
                        ),
                        dst: AnchoredPathBuf {
                            anchor: FileId(
                                1,
                            ),
                            path: "../foo2",
                        },
                    },
                ]
            "#]] ,) ; } # [test] fn test_rename_unusually_nested_mod () { check_expect ("bar" , r#"
//- /lib.rs
mod outer { mod fo$0o; }

//- /outer/foo.rs
// empty
"# , expect ! [[r#"
                source_file_edits: [
                    (
                        FileId(
                            0,
                        ),
                        [
                            Indel {
                                insert: "bar",
                                delete: 16..19,
                            },
                        ],
                    ),
                ]
                file_system_edits: [
                    MoveFile {
                        src: FileId(
                            1,
                        ),
                        dst: AnchoredPathBuf {
                            anchor: FileId(
                                1,
                            ),
                            path: "bar.rs",
                        },
                    },
                ]
            "#]] ,) ; } # [test] fn test_module_rename_in_path () { check ("baz" , r#"
mod $0foo {
    pub use self::bar as qux;
    pub fn bar() {}
}

fn main() { foo::bar(); }
"# , r#"
mod baz {
    pub use self::bar as qux;
    pub fn bar() {}
}

fn main() { baz::bar(); }
"# ,) ; } # [test] fn test_rename_mod_filename_and_path () { check_expect ("foo2" , r#"
//- /lib.rs
mod bar;
fn f() {
    bar::foo::fun()
}

//- /bar.rs
pub mod foo$0;

//- /bar/foo.rs
// pub fn fun() {}
"# , expect ! [[r#"
                source_file_edits: [
                    (
                        FileId(
                            0,
                        ),
                        [
                            Indel {
                                insert: "foo2",
                                delete: 27..30,
                            },
                        ],
                    ),
                    (
                        FileId(
                            1,
                        ),
                        [
                            Indel {
                                insert: "foo2",
                                delete: 8..11,
                            },
                        ],
                    ),
                ]
                file_system_edits: [
                    MoveFile {
                        src: FileId(
                            2,
                        ),
                        dst: AnchoredPathBuf {
                            anchor: FileId(
                                2,
                            ),
                            path: "foo2.rs",
                        },
                    },
                ]
            "#]] ,) ; } # [test] fn test_rename_mod_recursive () { check_expect ("foo2" , r#"
//- /lib.rs
mod foo$0;

//- /foo.rs
mod bar;
mod corge;

//- /foo/bar.rs
mod qux;

//- /foo/bar/qux.rs
mod quux;

//- /foo/bar/qux/quux/mod.rs
// empty

//- /foo/corge.rs
// empty
"# , expect ! [[r#"
                source_file_edits: [
                    (
                        FileId(
                            0,
                        ),
                        [
                            Indel {
                                insert: "foo2",
                                delete: 4..7,
                            },
                        ],
                    ),
                ]
                file_system_edits: [
                    MoveFile {
                        src: FileId(
                            1,
                        ),
                        dst: AnchoredPathBuf {
                            anchor: FileId(
                                1,
                            ),
                            path: "foo2.rs",
                        },
                    },
                    MoveDir {
                        src: AnchoredPathBuf {
                            anchor: FileId(
                                1,
                            ),
                            path: "foo",
                        },
                        src_id: FileId(
                            1,
                        ),
                        dst: AnchoredPathBuf {
                            anchor: FileId(
                                1,
                            ),
                            path: "foo2",
                        },
                    },
                ]
            "#]] ,) } # [test] fn test_rename_mod_ref_by_super () { check ("baz" , r#"
        mod $0foo {
        struct X;

        mod bar {
            use super::X;
        }
    }
            "# , r#"
        mod baz {
        struct X;

        mod bar {
            use super::X;
        }
    }
            "# ,) } # [test] fn test_rename_mod_in_macro () { check ("bar" , r#"
//- /foo.rs

//- /lib.rs
macro_rules! submodule {
    ($name:ident) => {
        mod $name;
    };
}

submodule!($0foo);
"# , r#"
macro_rules! submodule {
    ($name:ident) => {
        mod $name;
    };
}

submodule!(bar);
"# ,) } # [test] fn test_rename_mod_for_crate_root () { check_expect_will_rename_file ("main" , r#"
//- /lib.rs
use crate::foo as bar;
fn foo() {}
mod bar$0;
"# , expect ! [[r#"
                source_file_edits: []
                file_system_edits: []
            "#]] ,) } # [test] fn test_rename_mod_to_raw_ident () { check_expect ("r#fn" , r#"
//- /lib.rs
mod foo$0;

fn main() { foo::bar::baz(); }

//- /foo.rs
pub mod bar;

//- /foo/bar.rs
pub fn baz() {}
"# , expect ! [[r#"
                source_file_edits: [
                    (
                        FileId(
                            0,
                        ),
                        [
                            Indel {
                                insert: "r#fn",
                                delete: 4..7,
                            },
                            Indel {
                                insert: "r#fn",
                                delete: 22..25,
                            },
                        ],
                    ),
                ]
                file_system_edits: [
                    MoveFile {
                        src: FileId(
                            1,
                        ),
                        dst: AnchoredPathBuf {
                            anchor: FileId(
                                1,
                            ),
                            path: "fn.rs",
                        },
                    },
                    MoveDir {
                        src: AnchoredPathBuf {
                            anchor: FileId(
                                1,
                            ),
                            path: "foo",
                        },
                        src_id: FileId(
                            1,
                        ),
                        dst: AnchoredPathBuf {
                            anchor: FileId(
                                1,
                            ),
                            path: "fn",
                        },
                    },
                ]
            "#]] ,) ; } # [test] fn test_rename_mod_from_raw_ident () { check_expect ("foo" , r#"
//- /lib.rs
mod r#fn$0;

fn main() { r#fn::bar::baz(); }

//- /fn.rs
pub mod bar;

//- /fn/bar.rs
pub fn baz() {}
"# , expect ! [[r#"
                source_file_edits: [
                    (
                        FileId(
                            0,
                        ),
                        [
                            Indel {
                                insert: "foo",
                                delete: 4..8,
                            },
                            Indel {
                                insert: "foo",
                                delete: 23..27,
                            },
                        ],
                    ),
                ]
                file_system_edits: [
                    MoveFile {
                        src: FileId(
                            1,
                        ),
                        dst: AnchoredPathBuf {
                            anchor: FileId(
                                1,
                            ),
                            path: "foo.rs",
                        },
                    },
                    MoveDir {
                        src: AnchoredPathBuf {
                            anchor: FileId(
                                1,
                            ),
                            path: "fn",
                        },
                        src_id: FileId(
                            1,
                        ),
                        dst: AnchoredPathBuf {
                            anchor: FileId(
                                1,
                            ),
                            path: "foo",
                        },
                    },
                ]
            "#]] ,) ; } # [test] fn test_rename_each_usage_gets_appropriate_rawness () { check_expect ("dyn" , r#"
//- /a.rs crate:a edition:2015
pub fn foo() {}

//- /b.rs crate:b edition:2018 deps:a new_source_root:local
fn bar() {
    a::foo$0();
}
    "# , expect ! [[r#"
                source_file_edits: [
                    (
                        FileId(
                            0,
                        ),
                        [
                            Indel {
                                insert: "dyn",
                                delete: 7..10,
                            },
                        ],
                    ),
                    (
                        FileId(
                            1,
                        ),
                        [
                            Indel {
                                insert: "r#dyn",
                                delete: 18..21,
                            },
                        ],
                    ),
                ]
                file_system_edits: []
            "#]] ,) ; check_expect ("dyn" , r#"
//- /a.rs crate:a edition:2018
pub fn foo() {}

//- /b.rs crate:b edition:2015 deps:a new_source_root:local
fn bar() {
    a::foo$0();
}
    "# , expect ! [[r#"
                source_file_edits: [
                    (
                        FileId(
                            0,
                        ),
                        [
                            Indel {
                                insert: "r#dyn",
                                delete: 7..10,
                            },
                        ],
                    ),
                    (
                        FileId(
                            1,
                        ),
                        [
                            Indel {
                                insert: "dyn",
                                delete: 18..21,
                            },
                        ],
                    ),
                ]
                file_system_edits: []
            "#]] ,) ; check_expect ("r#dyn" , r#"
//- /a.rs crate:a edition:2018
pub fn foo$0() {}

//- /b.rs crate:b edition:2015 deps:a new_source_root:local
fn bar() {
    a::foo();
}
    "# , expect ! [[r#"
                source_file_edits: [
                    (
                        FileId(
                            0,
                        ),
                        [
                            Indel {
                                insert: "r#dyn",
                                delete: 7..10,
                            },
                        ],
                    ),
                    (
                        FileId(
                            1,
                        ),
                        [
                            Indel {
                                insert: "dyn",
                                delete: 18..21,
                            },
                        ],
                    ),
                ]
                file_system_edits: []
            "#]] ,) ; } # [test] fn rename_raw_identifier () { check_expect ("abc" , r#"
//- /a.rs crate:a edition:2015
pub fn dyn() {}

fn foo() {
    dyn$0();
}

//- /b.rs crate:b edition:2018 deps:a new_source_root:local
fn bar() {
    a::r#dyn();
}
    "# , expect ! [[r#"
                source_file_edits: [
                    (
                        FileId(
                            0,
                        ),
                        [
                            Indel {
                                insert: "abc",
                                delete: 7..10,
                            },
                            Indel {
                                insert: "abc",
                                delete: 32..35,
                            },
                        ],
                    ),
                    (
                        FileId(
                            1,
                        ),
                        [
                            Indel {
                                insert: "abc",
                                delete: 18..23,
                            },
                        ],
                    ),
                ]
                file_system_edits: []
            "#]] ,) ; check_expect ("abc" , r#"
//- /a.rs crate:a edition:2018
pub fn r#dyn() {}

fn foo() {
    r#dyn$0();
}

//- /b.rs crate:b edition:2015 deps:a new_source_root:local
fn bar() {
    a::dyn();
}
    "# , expect ! [[r#"
                source_file_edits: [
                    (
                        FileId(
                            0,
                        ),
                        [
                            Indel {
                                insert: "abc",
                                delete: 7..12,
                            },
                            Indel {
                                insert: "abc",
                                delete: 34..39,
                            },
                        ],
                    ),
                    (
                        FileId(
                            1,
                        ),
                        [
                            Indel {
                                insert: "abc",
                                delete: 18..21,
                            },
                        ],
                    ),
                ]
                file_system_edits: []
            "#]] ,) ; } # [test] fn test_enum_variant_from_module_1 () { cov_mark :: check ! (rename_non_local) ; check ("Baz" , r#"
mod foo {
    pub enum Foo { Bar$0 }
}

fn func(f: foo::Foo) {
    match f {
        foo::Foo::Bar => {}
    }
}
"# , r#"
mod foo {
    pub enum Foo { Baz }
}

fn func(f: foo::Foo) {
    match f {
        foo::Foo::Baz => {}
    }
}
"# ,) ; } # [test] fn test_enum_variant_from_module_2 () { check ("baz" , r#"
mod foo {
    pub struct Foo { pub bar$0: uint }
}

fn foo(f: foo::Foo) {
    let _ = f.bar;
}
"# , r#"
mod foo {
    pub struct Foo { pub baz: uint }
}

fn foo(f: foo::Foo) {
    let _ = f.baz;
}
"# ,) ; } # [test] fn test_parameter_to_self () { cov_mark :: check ! (rename_to_self) ; check ("self" , r#"
struct Foo { i: i32 }

impl Foo {
    fn f(foo$0: &mut Foo) -> i32 {
        foo.i
    }
}
"# , r#"
struct Foo { i: i32 }

impl Foo {
    fn f(&mut self) -> i32 {
        self.i
    }
}
"# ,) ; check ("self" , r#"
struct Foo { i: i32 }

impl Foo {
    fn f(foo$0: Foo) -> i32 {
        foo.i
    }
}
"# , r#"
struct Foo { i: i32 }

impl Foo {
    fn f(self) -> i32 {
        self.i
    }
}
"# ,) ; } # [test] fn test_parameter_to_self_error_no_impl () { check ("self" , r#"
struct Foo { i: i32 }

fn f(foo$0: &mut Foo) -> i32 {
    foo.i
}
"# , "error: Cannot rename parameter to self for free function" ,) ; check ("self" , r#"
struct Foo { i: i32 }
struct Bar;

impl Bar {
    fn f(foo$0: &mut Foo) -> i32 {
        foo.i
    }
}
"# , "error: Parameter type differs from impl block type" ,) ; } # [test] fn test_parameter_to_self_error_not_first () { check ("self" , r#"
struct Foo { i: i32 }
impl Foo {
    fn f(x: (), foo$0: &mut Foo) -> i32 {
        foo.i
    }
}
"# , "error: Only the first parameter may be renamed to self" ,) ; } # [test] fn test_parameter_to_self_impl_ref () { check ("self" , r#"
struct Foo { i: i32 }
impl &Foo {
    fn f(foo$0: &Foo) -> i32 {
        foo.i
    }
}
"# , r#"
struct Foo { i: i32 }
impl &Foo {
    fn f(self) -> i32 {
        self.i
    }
}
"# ,) ; } # [test] fn test_self_to_parameter () { check ("foo" , r#"
struct Foo { i: i32 }

impl Foo {
    fn f(&mut $0self) -> i32 {
        self.i
    }
}
"# , r#"
struct Foo { i: i32 }

impl Foo {
    fn f(foo: &mut Self) -> i32 {
        foo.i
    }
}
"# ,) ; } # [test] fn test_owned_self_to_parameter () { cov_mark :: check ! (rename_self_to_param) ; check ("foo" , r#"
struct Foo { i: i32 }

impl Foo {
    fn f($0self) -> i32 {
        self.i
    }
}
"# , r#"
struct Foo { i: i32 }

impl Foo {
    fn f(foo: Self) -> i32 {
        foo.i
    }
}
"# ,) ; } # [test] fn test_owned_self_to_parameter_with_lifetime () { cov_mark :: check ! (rename_self_to_param) ; check ("foo" , r#"
struct Foo<'a> { i: &'a i32 }

impl<'a> Foo<'a> {
    fn f(&'a $0self) -> i32 {
        self.i
    }
}
"# , r#"
struct Foo<'a> { i: &'a i32 }

impl<'a> Foo<'a> {
    fn f(foo: &'a Self) -> i32 {
        foo.i
    }
}
"# ,) ; } # [test] fn test_self_outside_of_methods () { check ("foo" , r#"
fn f($0self) -> i32 {
    self.i
}
"# , r#"
fn f(foo: Self) -> i32 {
    foo.i
}
"# ,) ; } # [test] fn no_type_value_ns_confuse () { check ("bar" , r#"
struct foo {}
fn f(foo$0: i32) -> i32 {
    use foo as _;
}
"# , r#"
struct foo {}
fn f(bar: i32) -> i32 {
    use foo as _;
}
"# ,) ; } # [test] fn test_self_in_path_to_parameter () { check ("foo" , r#"
struct Foo { i: i32 }

impl Foo {
    fn f(&self) -> i32 {
        let self_var = 1;
        self$0.i
    }
}
"# , r#"
struct Foo { i: i32 }

impl Foo {
    fn f(foo: &Self) -> i32 {
        let self_var = 1;
        foo.i
    }
}
"# ,) ; } # [test] fn test_rename_field_put_init_shorthand () { cov_mark :: check ! (test_rename_field_put_init_shorthand) ; check ("bar" , r#"
struct Foo { i$0: i32 }

fn foo(bar: i32) -> Foo {
    Foo { i: bar }
}
"# , r#"
struct Foo { bar: i32 }

fn foo(bar: i32) -> Foo {
    Foo { bar }
}
"# ,) ; } # [test] fn test_rename_local_simple () { check ("i" , r#"
fn foo(bar$0: i32) -> i32 {
    bar
}
"# , r#"
fn foo(i: i32) -> i32 {
    i
}
"# ,) ; } # [test] fn test_rename_local_put_init_shorthand () { cov_mark :: check ! (test_rename_local_put_init_shorthand) ; check ("i" , r#"
struct Foo { i: i32 }

fn foo(bar$0: i32) -> Foo {
    Foo { i: bar }
}
"# , r#"
struct Foo { i: i32 }

fn foo(i: i32) -> Foo {
    Foo { i }
}
"# ,) ; } # [test] fn test_struct_field_pat_into_shorthand () { cov_mark :: check ! (test_rename_field_put_init_shorthand_pat) ; check ("baz" , r#"
struct Foo { i$0: i32 }

fn foo(foo: Foo) {
    let Foo { i: ref baz @ qux } = foo;
    let _ = qux;
}
"# , r#"
struct Foo { baz: i32 }

fn foo(foo: Foo) {
    let Foo { baz: ref baz @ qux } = foo;
    let _ = qux;
}
"# ,) ; check ("baz" , r#"
struct Foo { i$0: i32 }

fn foo(foo: Foo) {
    let Foo { i: ref baz } = foo;
    let _ = qux;
}
"# , r#"
struct Foo { baz: i32 }

fn foo(foo: Foo) {
    let Foo { ref baz } = foo;
    let _ = qux;
}
"# ,) ; } # [test] fn test_struct_local_pat_into_shorthand () { cov_mark :: check ! (test_rename_local_put_init_shorthand_pat) ; check ("field" , r#"
struct Foo { field: i32 }

fn foo(foo: Foo) {
    let Foo { field: qux$0 } = foo;
    let _ = qux;
}
"# , r#"
struct Foo { field: i32 }

fn foo(foo: Foo) {
    let Foo { field } = foo;
    let _ = field;
}
"# ,) ; check ("field" , r#"
struct Foo { field: i32 }

fn foo(foo: Foo) {
    let Foo { field: x @ qux$0 } = foo;
    let _ = qux;
}
"# , r#"
struct Foo { field: i32 }

fn foo(foo: Foo) {
    let Foo { field: x @ field } = foo;
    let _ = field;
}
"# ,) ; } # [test] fn test_rename_binding_in_destructure_pat () { let expected_fixture = r#"
struct Foo {
    i: i32,
}

fn foo(foo: Foo) {
    let Foo { i: bar } = foo;
    let _ = bar;
}
"# ; check ("bar" , r#"
struct Foo {
    i: i32,
}

fn foo(foo: Foo) {
    let Foo { i: b } = foo;
    let _ = b$0;
}
"# , expected_fixture ,) ; check ("bar" , r#"
struct Foo {
    i: i32,
}

fn foo(foo: Foo) {
    let Foo { i } = foo;
    let _ = i$0;
}
"# , expected_fixture ,) ; } # [test] fn test_rename_binding_in_destructure_param_pat () { check ("bar" , r#"
struct Foo {
    i: i32
}

fn foo(Foo { i }: Foo) -> i32 {
    i$0
}
"# , r#"
struct Foo {
    i: i32
}

fn foo(Foo { i: bar }: Foo) -> i32 {
    bar
}
"# ,) } # [test] fn test_struct_field_complex_ident_pat () { cov_mark :: check ! (rename_record_pat_field_name_split) ; check ("baz" , r#"
struct Foo { i$0: i32 }

fn foo(foo: Foo) {
    let Foo { ref i } = foo;
}
"# , r#"
struct Foo { baz: i32 }

fn foo(foo: Foo) {
    let Foo { baz: ref i } = foo;
}
"# ,) ; } # [test] fn test_rename_lifetimes () { check ("'yeeee" , r#"
trait Foo<'a> {
    fn foo() -> &'a ();
}
impl<'a> Foo<'a> for &'a () {
    fn foo() -> &'a$0 () {
        unimplemented!()
    }
}
"# , r#"
trait Foo<'a> {
    fn foo() -> &'a ();
}
impl<'yeeee> Foo<'yeeee> for &'yeeee () {
    fn foo() -> &'yeeee () {
        unimplemented!()
    }
}
"# ,) } # [test] fn test_rename_bind_pat () { check ("new_name" , r#"
fn main() {
    enum CustomOption<T> {
        None,
        Some(T),
    }

    let test_variable = CustomOption::Some(22);

    match test_variable {
        CustomOption::Some(foo$0) if foo == 11 => {}
        _ => (),
    }
}"# , r#"
fn main() {
    enum CustomOption<T> {
        None,
        Some(T),
    }

    let test_variable = CustomOption::Some(22);

    match test_variable {
        CustomOption::Some(new_name) if new_name == 11 => {}
        _ => (),
    }
}"# ,) ; } # [test] fn test_rename_label () { check ("'foo" , r#"
fn foo<'a>() -> &'a () {
    'a: {
        'b: loop {
            break 'a$0;
        }
    }
}
"# , r#"
fn foo<'a>() -> &'a () {
    'foo: {
        'b: loop {
            break 'foo;
        }
    }
}
"# ,) } # [test] fn test_rename_label_new_name_without_apostrophe () { check ("foo" , r#"
fn main() {
    'outer$0: loop {
        'inner: loop {
            break 'outer;
        }
    }
}
        "# , r#"
fn main() {
    'foo: loop {
        'inner: loop {
            break 'foo;
        }
    }
}
        "# ,) ; } # [test] fn test_self_to_self () { cov_mark :: check ! (rename_self_to_self) ; check ("self" , r#"
struct Foo;
impl Foo {
    fn foo(self$0) {}
}
"# , r#"
struct Foo;
impl Foo {
    fn foo(self) {}
}
"# ,) } # [test] fn test_rename_field_in_pat_in_macro_doesnt_shorthand () { check ("baz" , r#"
macro_rules! foo {
    ($pattern:pat) => {
        let $pattern = loop {};
    };
}
struct Foo {
    bar$0: u32,
}
fn foo() {
    foo!(Foo { bar: baz });
}
"# , r#"
macro_rules! foo {
    ($pattern:pat) => {
        let $pattern = loop {};
    };
}
struct Foo {
    baz: u32,
}
fn foo() {
    foo!(Foo { baz: baz });
}
"# ,) } # [test] fn test_rename_tuple_field () { check ("foo" , r#"
struct Foo(i32);

fn baz() {
    let mut x = Foo(4);
    x.0$0 = 5;
}
"# , "error: No references found at position" ,) ; } # [test] fn test_rename_builtin () { check ("foo" , r#"
fn foo() {
    let x: i32$0 = 0;
}
"# , "error: Cannot rename builtin type" ,) ; } # [test] fn test_rename_self () { check ("foo" , r#"
struct Foo {}

impl Foo {
    fn foo(self) -> Self$0 {
        self
    }
}
"# , "error: No references found at position" ,) ; } # [test] fn test_rename_ignores_self_ty () { check ("Fo0" , r#"
struct $0Foo;

impl Foo where Self: {}
"# , r#"
struct Fo0;

impl Fo0 where Self: {}
"# ,) ; } # [test] fn test_rename_fails_on_aliases () { check ("Baz" , r#"
struct Foo;
use Foo as Bar$0;
"# , "error: Renaming aliases is currently unsupported" ,) ; check ("Baz" , r#"
struct Foo;
use Foo as Bar;
use Bar$0;
"# , "error: Renaming aliases is currently unsupported" ,) ; } # [test] fn test_rename_trait_method () { let res = r"
trait Foo {
    fn foo(&self) {
        self.foo();
    }
}

impl Foo for () {
    fn foo(&self) {
        self.foo();
    }
}" ; check ("foo" , r#"
trait Foo {
    fn bar$0(&self) {
        self.bar();
    }
}

impl Foo for () {
    fn bar(&self) {
        self.bar();
    }
}"# , res ,) ; check ("foo" , r#"
trait Foo {
    fn bar(&self) {
        self.bar$0();
    }
}

impl Foo for () {
    fn bar(&self) {
        self.bar();
    }
}"# , res ,) ; check ("foo" , r#"
trait Foo {
    fn bar(&self) {
        self.bar();
    }
}

impl Foo for () {
    fn bar$0(&self) {
        self.bar();
    }
}"# , res ,) ; check ("foo" , r#"
trait Foo {
    fn bar(&self) {
        self.bar();
    }
}

impl Foo for () {
    fn bar(&self) {
        self.bar$0();
    }
}"# , res ,) ; } # [test] fn test_rename_trait_method_prefix_of_second () { check ("qux" , r#"
trait Foo {
    fn foo$0() {}
    fn foobar() {}
}
"# , r#"
trait Foo {
    fn qux() {}
    fn foobar() {}
}
"# ,) ; } # [test] fn test_rename_trait_const () { let res = r"
trait Foo {
    const FOO: ();
}

impl Foo for () {
    const FOO: ();
}
fn f() { <()>::FOO; }" ; check ("FOO" , r#"
trait Foo {
    const BAR$0: ();
}

impl Foo for () {
    const BAR: ();
}
fn f() { <()>::BAR; }"# , res ,) ; check ("FOO" , r#"
trait Foo {
    const BAR: ();
}

impl Foo for () {
    const BAR$0: ();
}
fn f() { <()>::BAR; }"# , res ,) ; check ("FOO" , r#"
trait Foo {
    const BAR: ();
}

impl Foo for () {
    const BAR: ();
}
fn f() { <()>::BAR$0; }"# , res ,) ; } # [test] fn defs_from_macros_arent_renamed () { check ("lol" , r#"
macro_rules! m { () => { fn f() {} } }
m!();
fn main() { f$0()  }
"# , "error: No identifier available to rename" ,) } # [test] fn attributed_item () { check ("function" , r#"
//- proc_macros: identity

#[proc_macros::identity]
fn func$0() {
    func();
}
"# , r#"

#[proc_macros::identity]
fn function() {
    function();
}
"# ,) } # [test] fn in_macro_multi_mapping () { check ("a" , r#"
fn foo() {
    macro_rules! match_ast2 {
        ($node:ident {
            $( $res:expr, )*
        }) => {{
            $( if $node { $res } else )*
            { loop {} }
        }};
    }
    let $0d = 3;
    match_ast2! {
        d {
            d,
            d,
        }
    };
}
"# , r#"
fn foo() {
    macro_rules! match_ast2 {
        ($node:ident {
            $( $res:expr, )*
        }) => {{
            $( if $node { $res } else )*
            { loop {} }
        }};
    }
    let a = 3;
    match_ast2! {
        a {
            a,
            a,
        }
    };
}
"# ,) } # [test] fn rename_multi_local () { check ("bar" , r#"
fn foo((foo$0 | foo | foo): ()) {
    foo;
    let foo;
}
"# , r#"
fn foo((bar | bar | bar): ()) {
    bar;
    let foo;
}
"# ,) ; check ("bar" , r#"
fn foo((foo | foo$0 | foo): ()) {
    foo;
    let foo;
}
"# , r#"
fn foo((bar | bar | bar): ()) {
    bar;
    let foo;
}
"# ,) ; check ("bar" , r#"
fn foo((foo | foo | foo): ()) {
    foo$0;
    let foo;
}
"# , r#"
fn foo((bar | bar | bar): ()) {
    bar;
    let foo;
}
"# ,) ; } # [test] fn regression_13498 () { check ("Testing" , r"
mod foo {
    pub struct Test$0;
}

use foo::Test as Tester;

fn main() {
    let t = Tester;
}
" , r"
mod foo {
    pub struct Testing;
}

use foo::Testing as Tester;

fn main() {
    let t = Tester;
}
" ,) } # [test] fn extern_crate () { check_prepare (r"
//- /lib.rs crate:main deps:foo
extern crate foo$0;
use foo as qux;
//- /foo.rs crate:foo
" , expect ! [[r#"No references found at position"#]] ,) ; } # [test] fn extern_crate_rename () { check_prepare (r"
//- /lib.rs crate:main deps:foo
extern crate foo as qux$0;
use qux as frob;
//- /foo.rs crate:foo
" , expect ! ["Renaming aliases is currently unsupported"] ,) ; } # [test] fn extern_crate_self () { check_prepare (r"
extern crate self$0;
use self as qux;
" , expect ! ["No references found at position"] ,) ; } # [test] fn extern_crate_self_rename () { check_prepare (r"
//- /lib.rs crate:main deps:foo
extern crate self as qux$0;
use qux as frob;
//- /foo.rs crate:foo
" , expect ! ["Renaming aliases is currently unsupported"] ,) ; } # [test] fn disallow_renaming_for_non_local_definition () { check ("Baz" , r#"
//- /lib.rs crate:lib new_source_root:library
pub struct S;
//- /main.rs crate:main deps:lib new_source_root:local
use lib::S;
fn main() { let _: S$0; }
"# , "error: Cannot rename a non-local definition" ,) ; } # [test] fn disallow_renaming_for_builtin_macros () { check ("Baz" , r#"
//- minicore: derive, hash
//- /main.rs crate:main
use core::hash::Hash;
#[derive(H$0ash)]
struct A;
            "# , "error: Cannot rename a non-local definition" ,) ; } # [test] fn implicit_format_args () { check ("fbar" , r#"
//- minicore: fmt
fn test() {
    let foo = "foo";
    format_args!("hello {foo} {foo$0} {}", foo);
}
"# , r#"
fn test() {
    let fbar = "foo";
    format_args!("hello {fbar} {fbar} {}", fbar);
}
"# ,) ; } # [test] fn implicit_format_args2 () { check ("fo" , r#"
//- minicore: fmt
fn test() {
    let foo = "foo";
    format_args!("hello {foo} {foo$0} {}", foo);
}
"# , r#"
fn test() {
    let fo = "foo";
    format_args!("hello {fo} {fo} {}", fo);
}
"# ,) ; } # [test] fn asm_operand () { check ("bose" , r#"
//- minicore: asm
fn test() {
    core::arch::asm!(
        "push {base}",
        base$0 = const 0
    );
}
"# , r#"
fn test() {
    core::arch::asm!(
        "push {bose}",
        bose = const 0
    );
}
"# ,) ; } # [test] fn asm_operand2 () { check ("bose" , r#"
//- minicore: asm
fn test() {
    core::arch::asm!(
        "push {base$0}",
        "push {base}",
        boo = const 0,
        virtual_free = sym VIRTUAL_FREE,
        base = const 0,
        boo = const 0,
    );
}
"# , r#"
fn test() {
    core::arch::asm!(
        "push {bose}",
        "push {bose}",
        boo = const 0,
        virtual_free = sym VIRTUAL_FREE,
        bose = const 0,
        boo = const 0,
    );
}
"# ,) ; } # [test] fn rename_path_inside_use_tree () { check ("Baz" , r#"
//- /main.rs crate:main
mod module;
mod foo { pub struct Foo; }
mod bar { use super::Foo; }

use foo::Foo$0;

fn main() { let _: Foo; }
//- /module.rs
use crate::foo::Foo;
"# , r#"
mod module;
mod foo { pub struct Foo; }
mod bar { use super::Baz; }

use foo::Foo as Baz;

fn main() { let _: Baz; }
"# ,) } # [test] fn rename_path_inside_use_tree_foreign () { check ("Baz" , r#"
//- /lib.rs crate:lib new_source_root:library
pub struct S;
//- /main.rs crate:main deps:lib new_source_root:local
use lib::S$0;
fn main() { let _: S; }
"# , r#"
use lib::S as Baz;
fn main() { let _: Baz; }
"# ,) ; } # [test] fn rename_type_param_ref_in_use_bound () { check ("U" , r#"
fn foo<T>() -> impl use<T$0> Trait {}
"# , r#"
fn foo<U>() -> impl use<U> Trait {}
"# ,) ; } # [test] fn rename_type_param_in_use_bound () { check ("U" , r#"
fn foo<T$0>() -> impl use<T> Trait {}
"# , r#"
fn foo<U>() -> impl use<U> Trait {}
"# ,) ; } # [test] fn rename_lifetime_param_ref_in_use_bound () { check ("u" , r#"
fn foo<'t>() -> impl use<'t$0> Trait {}
"# , r#"
fn foo<'u>() -> impl use<'u> Trait {}
"# ,) ; } # [test] fn rename_lifetime_param_in_use_bound () { check ("u" , r#"
fn foo<'t$0>() -> impl use<'t> Trait {}
"# , r#"
fn foo<'u>() -> impl use<'u> Trait {}
"# ,) ; } # [test] fn rename_parent_type_param_in_use_bound () { check ("U" , r#"
trait Trait<T> {
    fn foo() -> impl use<T$0> Trait {}
}
"# , r#"
trait Trait<U> {
    fn foo() -> impl use<U> Trait {}
}
"# ,) ; } # [test] fn rename_macro_generated_type_from_type_with_a_suffix () { check ("Bar" , r#"
//- proc_macros: generate_suffixed_type
#[proc_macros::generate_suffixed_type]
struct Foo$0;
fn usage(_: FooSuffix) {}
usage(FooSuffix);
"# , r#"
#[proc_macros::generate_suffixed_type]
struct Bar;
fn usage(_: BarSuffix) {}
usage(BarSuffix);
"# ,) ; } # [test] # [should_panic] fn rename_macro_generated_type_from_type_usage_with_a_suffix () { check ("Bar" , r#"
//- proc_macros: generate_suffixed_type
#[proc_macros::generate_suffixed_type]
struct Foo;
fn usage(_: FooSuffix) {}
usage(FooSuffix);
fn other_place() { Foo$0; }
"# , r#"
#[proc_macros::generate_suffixed_type]
struct Bar;
fn usage(_: BarSuffix) {}
usage(BarSuffix);
fn other_place() { Bar; }
"# ,) ; } # [test] fn rename_macro_generated_type_from_variant_with_a_suffix () { check ("Bar" , r#"
//- proc_macros: generate_suffixed_type
#[proc_macros::generate_suffixed_type]
enum Quux {
    Foo$0,
}
fn usage(_: FooSuffix) {}
usage(FooSuffix);
"# , r#"
#[proc_macros::generate_suffixed_type]
enum Quux {
    Bar,
}
fn usage(_: BarSuffix) {}
usage(BarSuffix);
"# ,) ; } # [test] # [should_panic] fn rename_macro_generated_type_from_variant_usage_with_a_suffix () { check ("Bar" , r#"
//- proc_macros: generate_suffixed_type
#[proc_macros::generate_suffixed_type]
enum Quux {
    Foo,
}
fn usage(_: FooSuffix) {}
usage(FooSuffix);
fn other_place() { Quux::Foo$0; }
"# , r#"
#[proc_macros::generate_suffixed_type]
enum Quux {
    Bar,
}
fn usage(_: BarSuffix) {}
usage(BartSuffix);
fn other_place() { Quux::Bar$0; }
"# ,) ; } # [test] fn rename_to_self_callers () { check ("self" , r#"
//- minicore: add
struct Foo;
impl core::ops::Add for Foo {
    type Target = Foo;
    fn add(self, _: Self) -> Foo { Foo }
}

impl Foo {
    fn foo(th$0is: &Self) {}
}

fn bar(v: &Foo) {
    Foo::foo(v);
}

fn baz() {
    Foo::foo(&Foo);
    Foo::foo(Foo + Foo);
}
        "# , r#"
struct Foo;
impl core::ops::Add for Foo {
    type Target = Foo;
    fn add(self, _: Self) -> Foo { Foo }
}

impl Foo {
    fn foo(&self) {}
}

fn bar(v: &Foo) {
    v.foo();
}

fn baz() {
    Foo.foo();
    (Foo + Foo).foo();
}
        "# ,) ; check ("self" , r#"
struct Foo;

impl Foo {
    fn foo(th$0is: &Self, v: i32) {}
}

fn bar(v: Foo) {
    Foo::foo(&v, 123);
}
        "# , r#"
struct Foo;

impl Foo {
    fn foo(&self, v: i32) {}
}

fn bar(v: Foo) {
    v.foo(123);
}
        "# ,) ; } # [test] fn rename_to_self_callers_in_macro () { check ("self" , r#"
struct Foo;

impl Foo {
    fn foo(th$0is: &Self, v: i32) {}
}

macro_rules! m { ($it:expr) => { $it } }
fn bar(v: Foo) {
    m!(Foo::foo(&v, 123));
}
        "# , r#"
struct Foo;

impl Foo {
    fn foo(&self, v: i32) {}
}

macro_rules! m { ($it:expr) => { $it } }
fn bar(v: Foo) {
    m!(v.foo( 123));
}
        "# ,) ; } # [test] fn rename_from_self_callers () { check ("this" , r#"
//- minicore: add
struct Foo;
impl Foo {
    fn foo(&sel$0f) {}
}
impl core::ops::Add for Foo {
    type Output = Foo;

    fn add(self, _rhs: Self) -> Self::Output {
        Foo
    }
}

fn bar(v: &Foo) {
    v.foo();
    (Foo + Foo).foo();
}

mod baz {
    fn baz(v: super::Foo) {
        v.foo();
    }
}
        "# , r#"
struct Foo;
impl Foo {
    fn foo(this: &Self) {}
}
impl core::ops::Add for Foo {
    type Output = Foo;

    fn add(self, _rhs: Self) -> Self::Output {
        Foo
    }
}

fn bar(v: &Foo) {
    Foo::foo(v);
    Foo::foo(&(Foo + Foo));
}

mod baz {
    fn baz(v: super::Foo) {
        crate::Foo::foo(&v);
    }
}
        "# ,) ; check ("this" , r#"
struct Foo;
impl Foo {
    fn foo(&sel$0f, _v: i32) {}
}

fn bar() {
    Foo.foo(1);
}
        "# , r#"
struct Foo;
impl Foo {
    fn foo(this: &Self, _v: i32) {}
}

fn bar() {
    Foo::foo(&Foo, 1);
}
        "# ,) ; } }
};
}

// Generated macro for tests (module)
macro_rules! Depcrate_view_memory_layouttests {
() => {
// Module: crate::view_memory_layout
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; use crate :: fixture ; use expect_test :: expect ; fn make_memory_layout (# [rust_analyzer :: rust_fixture] ra_fixture : & str ,) -> Option < RecursiveMemoryLayout > { let (analysis , position , _) = fixture :: annotations (ra_fixture) ; hir :: attach_db (& analysis . db , | | view_memory_layout (& analysis . db , position)) } # [test] fn view_memory_layout_none () { assert ! (make_memory_layout (r#"$0"#) . is_none ()) ; assert ! (make_memory_layout (r#"stru$0ct Blah {}"#) . is_none ()) ; } # [test] fn view_memory_layout_primitive () { expect ! [[r#"
            foo: i32 (size: 4, align: 4, field offset: 0)
        "#]] . assert_eq (& make_memory_layout (r#"
fn main() {
    let foo$0 = 109; // default i32
}
"# ,) . unwrap () . to_string () ,) ; } # [test] fn view_memory_layout_constant () { expect ! [[r#"
            BLAH: bool (size: 1, align: 1, field offset: 0)
        "#]] . assert_eq (& make_memory_layout (r#"
const BLAH$0: bool = 0;
"# ,) . unwrap () . to_string () ,) ; } # [test] fn view_memory_layout_static () { expect ! [[r#"
            BLAH: bool (size: 1, align: 1, field offset: 0)
        "#]] . assert_eq (& make_memory_layout (r#"
static BLAH$0: bool = 0;
"# ,) . unwrap () . to_string () ,) ; } # [test] fn view_memory_layout_tuple () { expect ! [[r#"
            x: (f64, u8, i64) (size: 24, align: 8, field offset: 0)
            	.0: f64 (size: 8, align: 8, field offset: 0)
            	.1: u8 (size: 1, align: 1, field offset: 8)
            	.2: i64 (size: 8, align: 8, field offset: 16)
        "#]] . assert_eq (& make_memory_layout (r#"
fn main() {
    let x$0 = (101.0, 111u8, 119i64);
}
"# ,) . unwrap () . to_string () ,) ; } # [test] fn view_memory_layout_c_struct () { expect ! [[r#"
            [ROOT]: Blah (size: 16, align: 4, field offset: 0)
            	a: u32 (size: 4, align: 4, field offset: 0)
            	b: (i32, u8) (size: 8, align: 4, field offset: 4)
            		.0: i32 (size: 4, align: 4, field offset: 0)
            		.1: u8 (size: 1, align: 1, field offset: 4)
            	c: i8 (size: 1, align: 1, field offset: 12)
        "#]] . assert_eq (& make_memory_layout (r#"
#[repr(C)]
struct Blah$0 {
    a: u32,
    b: (i32, u8),
    c: i8,
}
"# ,) . unwrap () . to_string () ,) ; } # [test] fn view_memory_layout_struct () { expect ! [[r#"
            [ROOT]: Blah (size: 16, align: 4, field offset: 0)
            	b: (i32, u8) (size: 8, align: 4, field offset: 0)
            		.0: i32 (size: 4, align: 4, field offset: 0)
            		.1: u8 (size: 1, align: 1, field offset: 4)
            	a: u32 (size: 4, align: 4, field offset: 8)
            	c: i8 (size: 1, align: 1, field offset: 12)
        "#]] . assert_eq (& make_memory_layout (r#"
struct Blah$0 {
    a: u32,
    b: (i32, u8),
    c: i8,
}
"# ,) . unwrap () . to_string () ,) ; } # [test] fn view_memory_layout_member () { expect ! [[r#"
            a: bool (size: 1, align: 1, field offset: 0)
        "#]] . assert_eq (& make_memory_layout (r#"
#[repr(C)]
struct Oof {
    a$0: bool,
}
"# ,) . unwrap () . to_string () ,) ; } # [test] fn view_memory_layout_alias () { let ml_a = make_memory_layout (r#"
struct X {
    a: u32,
    b: i8,
    c: (f32, f32),
}

type Foo$0 = X;
"# ,) . unwrap () ; let ml_b = make_memory_layout (r#"
struct X$0 {
    a: u32,
    b: i8,
    c: (f32, f32),
}
"# ,) . unwrap () ; assert_eq ! (ml_a . to_string () , ml_b . to_string ()) ; } }
};
}

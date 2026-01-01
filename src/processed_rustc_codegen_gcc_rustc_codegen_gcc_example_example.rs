// SRC: ../rust/compiler/rustc_codegen_gcc/example/example.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=abc | COMPLEXITY=2 | LINES=10 */
#[feature(no_core, unboxed_closures)]
#[no_core]
#[allow(dead_code, unnecessary_transmutes)]


use mini_core::*;

fn abc(a: u8) -> u8 {
    a * 2
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=bcd | COMPLEXITY=6 | LINES=4 */

fn bcd(b: bool, a: u8) -> u8 {
    if b { a * 2 } else { a * 3 }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=call | COMPLEXITY=2 | LINES=4 */

fn call() {
    abc(42);
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=indirect_call | COMPLEXITY=2 | LINES=5 */

fn indirect_call() {
    let f: fn() = call;
    f();
}
/* AST_META: AST_ID=5 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */

enum BoolOption {
    Some(bool),
    None,
}
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=option_unwrap_or | COMPLEXITY=6 | LINES=7 */

fn option_unwrap_or(o: BoolOption, d: bool) -> bool {
    match o {
        BoolOption::Some(b) => b,
        BoolOption::None => d,
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=ret_42 | COMPLEXITY=2 | LINES=4 */

fn ret_42() -> u8 {
    42
}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=return_str | COMPLEXITY=2 | LINES=4 */

fn return_str() -> &'static str {
    "hello world"
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=promoted_val | COMPLEXITY=2 | LINES=4 */

fn promoted_val() -> &'static u8 {
    &(1 * 2)
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=cast_ref_to_raw_ptr | COMPLEXITY=2 | LINES=4 */

fn cast_ref_to_raw_ptr(abc: &u8) -> *const u8 {
    abc as *const u8
}
/* AST_META: AST_ID=11 | TYPE=FUNCTION | NAME=cmp_raw_ptr | COMPLEXITY=2 | LINES=4 */

fn cmp_raw_ptr(a: *const u8, b: *const u8) -> bool {
    a == b
}
/* AST_META: AST_ID=12 | TYPE=FUNCTION | NAME=int_cast | COMPLEXITY=2 | LINES=7 */

fn int_cast(a: u16, b: i16) -> (u8, u16, u32, usize, i8, i16, i32, isize, u8, u32) {
    (
        a as u8, a as u16, a as u32, a as usize, a as i8, a as i16, a as i32, a as isize, b as u8,
        b as u32,
    )
}
/* AST_META: AST_ID=13 | TYPE=FUNCTION | NAME=char_cast | COMPLEXITY=2 | LINES=4 */

fn char_cast(c: char) -> u8 {
    c as u8
}
/* AST_META: AST_ID=14 | TYPE=FUNCTION | NAME=DebugTuple(()); | COMPLEXITY=2 | LINES=6 */

pub struct DebugTuple(());

fn debug_tuple() -> DebugTuple {
    DebugTuple(())
}
/* AST_META: AST_ID=15 | TYPE=FUNCTION | NAME=size_of | COMPLEXITY=2 | LINES=4 */

fn size_of<T>() -> usize {
    intrinsics::size_of::<T>()
}
/* AST_META: AST_ID=16 | TYPE=FUNCTION | NAME=use_size_of | COMPLEXITY=2 | LINES=4 */

fn use_size_of() -> usize {
    size_of::<u64>()
}
/* AST_META: AST_ID=17 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=4 */

unsafe fn use_copy_intrinsic(src: *const u8, dst: *mut u8) {
    intrinsics::copy::<u8>(src, dst, 1);
}
/* AST_META: AST_ID=18 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=5 */

unsafe fn use_copy_intrinsic_ref(src: *const u8, dst: *mut u8) {
    let copy2 = &intrinsics::copy::<u8>;
    copy2(src, dst, 1);
}
/* AST_META: AST_ID=19 | TYPE=FUNCTION | NAME=use_const | COMPLEXITY=2 | LINES=6 */

const ABC: u8 = 6 * 7;

fn use_const() -> u8 {
    ABC
}
/* AST_META: AST_ID=20 | TYPE=FUNCTION | NAME=call_closure_3arg | COMPLEXITY=3 | LINES=4 */

pub fn call_closure_3arg() {
    (|_, _, _| {})(0u8, 42u16, 0u8)
}
/* AST_META: AST_ID=21 | TYPE=FUNCTION | NAME=call_closure_2arg | COMPLEXITY=3 | LINES=4 */

pub fn call_closure_2arg() {
    (|_, _| {})(0u8, 42u16)
}
/* AST_META: AST_ID=22 | TYPE=FUNCTION | NAME=IsNotEmpty; | COMPLEXITY=5 | LINES=11 */

struct IsNotEmpty;

impl<'a, 'b> FnOnce<(&'a &'b [u16],)> for IsNotEmpty {
    type Output = (u8, u8);

    #[inline]
    extern "rust-call" fn call_once(mut self, arg: (&'a &'b [u16],)) -> (u8, u8) {
        self.call_mut(arg)
    }
}
/* AST_META: AST_ID=23 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=5 | LINES=7 */

impl<'a, 'b> FnMut<(&'a &'b [u16],)> for IsNotEmpty {
    #[inline]
    extern "rust-call" fn call_mut(&mut self, _arg: (&'a &'b [u16],)) -> (u8, u8) {
        (0, 42)
    }
}
/* AST_META: AST_ID=24 | TYPE=FUNCTION | NAME=call_is_not_empty | COMPLEXITY=2 | LINES=4 */

pub fn call_is_not_empty() {
    IsNotEmpty.call_once((&(&[0u16] as &[_]),));
}
/* AST_META: AST_ID=25 | TYPE=FUNCTION | NAME=eq_char | COMPLEXITY=2 | LINES=4 */

fn eq_char(a: char, b: char) -> bool {
    a == b
}
/* AST_META: AST_ID=26 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=4 */

unsafe fn transmute(c: char) -> u32 {
    intrinsics::transmute(c)
}
/* AST_META: AST_ID=27 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=4 */

unsafe fn deref_str_ptr(s: *const str) -> &'static str {
    &*s
}
/* AST_META: AST_ID=28 | TYPE=FUNCTION | NAME=use_array | COMPLEXITY=2 | LINES=4 */

fn use_array(arr: [u8; 3]) -> u8 {
    arr[1]
}
/* AST_META: AST_ID=29 | TYPE=FUNCTION | NAME=repeat_array | COMPLEXITY=2 | LINES=4 */

fn repeat_array() -> [u8; 3] {
    [0; 3]
}
/* AST_META: AST_ID=30 | TYPE=FUNCTION | NAME=array_as_slice | COMPLEXITY=2 | LINES=4 */

fn array_as_slice(arr: &[u8; 3]) -> &[u8] {
    arr
}
/* AST_META: AST_ID=31 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=4 */

unsafe fn use_ctlz_nonzero(a: u16) -> u32 {
    intrinsics::ctlz_nonzero(a)
}
/* AST_META: AST_ID=32 | TYPE=FUNCTION | NAME=ptr_as_usize | COMPLEXITY=2 | LINES=4 */

fn ptr_as_usize(ptr: *const u8) -> usize {
    ptr as usize
}
/* AST_META: AST_ID=33 | TYPE=FUNCTION | NAME=float_cast | COMPLEXITY=2 | LINES=4 */

fn float_cast(a: f32, b: f64) -> (f64, f32) {
    (a as f64, b as f32)
}
/* AST_META: AST_ID=34 | TYPE=FUNCTION | NAME=int_to_float | COMPLEXITY=2 | LINES=4 */

fn int_to_float(a: u8, b: i32) -> (f64, f32) {
    (a as f64, b as f32)
}
/* AST_META: AST_ID=35 | TYPE=FUNCTION | NAME=make_array | COMPLEXITY=2 | LINES=4 */

fn make_array() -> [u8; 3] {
    [42, 0, 5]
}
/* AST_META: AST_ID=36 | TYPE=FUNCTION | NAME=some_promoted_tuple | COMPLEXITY=2 | LINES=4 */

fn some_promoted_tuple() -> &'static (&'static str, &'static str) {
    &("abc", "some")
}
/* AST_META: AST_ID=37 | TYPE=FUNCTION | NAME=index_slice | COMPLEXITY=2 | LINES=4 */

fn index_slice(s: &[u8]) -> u8 {
    s[2]
}
/* AST_META: AST_ID=38 | TYPE=STRUCT | NAME=StrWrapper | COMPLEXITY=2 | LINES=4 */

pub struct StrWrapper {
    s: str,
}
/* AST_META: AST_ID=39 | TYPE=FUNCTION | NAME=str_wrapper_get | COMPLEXITY=2 | LINES=4 */

fn str_wrapper_get(w: &StrWrapper) -> &str {
    &w.s
}
/* AST_META: AST_ID=40 | TYPE=FUNCTION | NAME=i16_as_i8 | COMPLEXITY=2 | LINES=4 */

fn i16_as_i8(a: i16) -> i8 {
    a as i8
}
/* AST_META: AST_ID=41 | TYPE=FUNCTION | NAME=Unsized(u8, | COMPLEXITY=2 | LINES=6 */

struct Unsized(u8, str);

fn get_sized_field_ref_from_unsized_type(u: &Unsized) -> &u8 {
    &u.0
}
/* AST_META: AST_ID=42 | TYPE=FUNCTION | NAME=get_unsized_field_ref_from_unsized_type | COMPLEXITY=2 | LINES=4 */

fn get_unsized_field_ref_from_unsized_type(u: &Unsized) -> &str {
    &u.1
}
/* AST_META: AST_ID=43 | TYPE=FUNCTION | NAME=reuse_byref_argument_storage | COMPLEXITY=2 | LINES=4 */

pub fn reuse_byref_argument_storage(a: (u8, u16, u32)) -> u8 {
    a.0
}
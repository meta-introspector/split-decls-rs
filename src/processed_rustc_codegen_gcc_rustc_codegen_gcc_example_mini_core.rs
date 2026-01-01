// SRC: ../rust/compiler/rustc_codegen_gcc/example/mini_core.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=20 */
#[feature(
    no_core,
    lang_items,
    intrinsics,
    unboxed_closures,
    extern_types,
    decl_macro,
    rustc_attrs,
    transparent_unions,
    auto_traits,
    freeze_impls,
    thread_local
)]
#[no_core]
#[allow(dead_code, internal_features, ambiguous_wide_pointer_comparisons)]

#[unsafe(no_mangle)]
unsafe extern "C" fn _Unwind_Resume() {
    intrinsics::unreachable();
}
/* AST_META: AST_ID=2 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "pointee_sized"]
pub trait PointeeSized {}
/* AST_META: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "meta_sized"]
pub trait MetaSized: PointeeSized {}
/* AST_META: AST_ID=4 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "sized"]
pub trait Sized: MetaSized {}
/* AST_META: AST_ID=5 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "destruct"]
pub trait Destruct {}
/* AST_META: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "tuple_trait"]
pub trait Tuple {}
/* AST_META: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "unsize"]
pub trait Unsize<T: PointeeSized>: PointeeSized {}
/* AST_META: AST_ID=8 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "coerce_unsized"]
pub trait CoerceUnsized<T> {}
/* AST_META: AST_ID=9 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl<'a, 'b: 'a, T: PointeeSized + Unsize<U>, U: PointeeSized> CoerceUnsized<&'a U> for &'b T {}
/* AST_META: AST_ID=10 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl<'a, T: PointeeSized + Unsize<U>, U: PointeeSized> CoerceUnsized<&'a mut U> for &'a mut T {}
/* AST_META: AST_ID=11 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl<T: PointeeSized + Unsize<U>, U: PointeeSized> CoerceUnsized<*const U> for *const T {}
/* AST_META: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl<T: PointeeSized + Unsize<U>, U: PointeeSized> CoerceUnsized<*mut U> for *mut T {}
/* AST_META: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "dispatch_from_dyn"]
pub trait DispatchFromDyn<T> {}
/* AST_META: AST_ID=14 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=3 */

// &T -> &U
impl<'a, T: PointeeSized + Unsize<U>, U: PointeeSized> DispatchFromDyn<&'a U> for &'a T {}
/* AST_META: AST_ID=15 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */
// &mut T -> &mut U
impl<'a, T: PointeeSized + Unsize<U>, U: PointeeSized> DispatchFromDyn<&'a mut U> for &'a mut T {}
/* AST_META: AST_ID=16 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */
// *const T -> *const U
impl<T: PointeeSized + Unsize<U>, U: PointeeSized> DispatchFromDyn<*const U> for *const T {}
/* AST_META: AST_ID=17 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */
// *mut T -> *mut U
impl<T: PointeeSized + Unsize<U>, U: PointeeSized> DispatchFromDyn<*mut U> for *mut T {}
/* AST_META: AST_ID=18 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl<T: MetaSized + Unsize<U>, U: MetaSized> DispatchFromDyn<Box<U, ()>> for Box<T, ()> {}
/* AST_META: AST_ID=19 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "legacy_receiver"]
pub trait LegacyReceiver {}
/* AST_META: AST_ID=20 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl<T: PointeeSized> LegacyReceiver for &T {}
/* AST_META: AST_ID=21 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl<T: PointeeSized> LegacyReceiver for &mut T {}
/* AST_META: AST_ID=22 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl<T: MetaSized> LegacyReceiver for Box<T> {}
/* AST_META: AST_ID=23 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "receiver"]
trait Receiver {}
/* AST_META: AST_ID=24 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "copy"]
pub trait Copy {}
/* AST_META: AST_ID=25 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "bikeshed_guaranteed_no_drop"]
pub trait BikeshedGuaranteedNoDrop {}
/* AST_META: AST_ID=26 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl Copy for bool {}
/* AST_META: AST_ID=27 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for u8 {}
/* AST_META: AST_ID=28 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for u16 {}
/* AST_META: AST_ID=29 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for u32 {}
/* AST_META: AST_ID=30 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for u64 {}
/* AST_META: AST_ID=31 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for usize {}
/* AST_META: AST_ID=32 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for u128 {}
/* AST_META: AST_ID=33 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for i8 {}
/* AST_META: AST_ID=34 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for i16 {}
/* AST_META: AST_ID=35 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for i32 {}
/* AST_META: AST_ID=36 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for i64 {}
/* AST_META: AST_ID=37 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for isize {}
/* AST_META: AST_ID=38 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for i128 {}
/* AST_META: AST_ID=39 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for f32 {}
/* AST_META: AST_ID=40 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for f64 {}
/* AST_META: AST_ID=41 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl Copy for char {}
/* AST_META: AST_ID=42 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl<'a, T: PointeeSized> Copy for &'a T {}
/* AST_META: AST_ID=43 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl<T: PointeeSized> Copy for *const T {}
/* AST_META: AST_ID=44 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl<T: PointeeSized> Copy for *mut T {}
/* AST_META: AST_ID=45 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=6 | LINES=3 */

#[lang = "sync"]
pub unsafe trait Sync {}
/* AST_META: AST_ID=46 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=2 */

unsafe impl Sync for bool {}
/* AST_META: AST_ID=47 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl Sync for u8 {}
/* AST_META: AST_ID=48 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl Sync for u16 {}
/* AST_META: AST_ID=49 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl Sync for u32 {}
/* AST_META: AST_ID=50 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl Sync for u64 {}
/* AST_META: AST_ID=51 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl Sync for usize {}
/* AST_META: AST_ID=52 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl Sync for i8 {}
/* AST_META: AST_ID=53 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl Sync for i16 {}
/* AST_META: AST_ID=54 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl Sync for i32 {}
/* AST_META: AST_ID=55 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl Sync for isize {}
/* AST_META: AST_ID=56 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl Sync for char {}
/* AST_META: AST_ID=57 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl<'a, T: PointeeSized> Sync for &'a T {}
/* AST_META: AST_ID=58 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl Sync for [u8; 16] {}
/* AST_META: AST_ID=59 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=6 | LINES=3 */

#[lang = "freeze"]
unsafe auto trait Freeze {}
/* AST_META: AST_ID=60 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=2 */

unsafe impl<T: PointeeSized> Freeze for PhantomData<T> {}
/* AST_META: AST_ID=61 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl<T: PointeeSized> Freeze for *const T {}
/* AST_META: AST_ID=62 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl<T: PointeeSized> Freeze for *mut T {}
/* AST_META: AST_ID=63 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl<T: PointeeSized> Freeze for &T {}
/* AST_META: AST_ID=64 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=8 | LINES=1 */
unsafe impl<T: PointeeSized> Freeze for &mut T {}
/* AST_META: AST_ID=65 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "structural_peq"]
pub trait StructuralPartialEq {}
/* AST_META: AST_ID=66 | TYPE=FUNCTION | NAME=not | COMPLEXITY=2 | LINES=7 */

#[lang = "not"]
pub trait Not {
    type Output;

    fn not(self) -> Self::Output;
}
/* AST_META: AST_ID=67 | TYPE=FUNCTION | NAME=not | COMPLEXITY=5 | LINES=8 */

impl Not for bool {
    type Output = bool;

    fn not(self) -> bool {
        !self
    }
}
/* AST_META: AST_ID=68 | TYPE=FUNCTION | NAME=mul | COMPLEXITY=2 | LINES=8 */

#[lang = "mul"]
pub trait Mul<RHS = Self> {
    type Output;

    #[must_use]
    fn mul(self, rhs: RHS) -> Self::Output;
}
/* AST_META: AST_ID=69 | TYPE=FUNCTION | NAME=mul | COMPLEXITY=5 | LINES=8 */

impl Mul for u8 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}
/* AST_META: AST_ID=70 | TYPE=FUNCTION | NAME=mul | COMPLEXITY=5 | LINES=8 */

impl Mul for i32 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}
/* AST_META: AST_ID=71 | TYPE=FUNCTION | NAME=mul | COMPLEXITY=5 | LINES=8 */

impl Mul for usize {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}
/* AST_META: AST_ID=72 | TYPE=FUNCTION | NAME=mul | COMPLEXITY=5 | LINES=8 */

impl Mul for isize {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        self * rhs
    }
}
/* AST_META: AST_ID=73 | TYPE=FUNCTION | NAME=add | COMPLEXITY=2 | LINES=7 */

#[lang = "add"]
pub trait Add<RHS = Self> {
    type Output;

    fn add(self, rhs: RHS) -> Self::Output;
}
/* AST_META: AST_ID=74 | TYPE=FUNCTION | NAME=add | COMPLEXITY=5 | LINES=8 */

impl Add for u8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}
/* AST_META: AST_ID=75 | TYPE=FUNCTION | NAME=add | COMPLEXITY=5 | LINES=8 */

impl Add for i8 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}
/* AST_META: AST_ID=76 | TYPE=FUNCTION | NAME=add | COMPLEXITY=5 | LINES=8 */

impl Add for i32 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}
/* AST_META: AST_ID=77 | TYPE=FUNCTION | NAME=add | COMPLEXITY=5 | LINES=8 */

impl Add for usize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}
/* AST_META: AST_ID=78 | TYPE=FUNCTION | NAME=add | COMPLEXITY=5 | LINES=8 */

impl Add for isize {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        self + rhs
    }
}
/* AST_META: AST_ID=79 | TYPE=FUNCTION | NAME=sub | COMPLEXITY=2 | LINES=7 */

#[lang = "sub"]
pub trait Sub<RHS = Self> {
    type Output;

    fn sub(self, rhs: RHS) -> Self::Output;
}
/* AST_META: AST_ID=80 | TYPE=FUNCTION | NAME=sub | COMPLEXITY=5 | LINES=8 */

impl Sub for usize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}
/* AST_META: AST_ID=81 | TYPE=FUNCTION | NAME=sub | COMPLEXITY=5 | LINES=8 */

impl Sub for isize {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}
/* AST_META: AST_ID=82 | TYPE=FUNCTION | NAME=sub | COMPLEXITY=5 | LINES=8 */

impl Sub for u8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}
/* AST_META: AST_ID=83 | TYPE=FUNCTION | NAME=sub | COMPLEXITY=5 | LINES=8 */

impl Sub for i8 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}
/* AST_META: AST_ID=84 | TYPE=FUNCTION | NAME=sub | COMPLEXITY=5 | LINES=8 */

impl Sub for i16 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}
/* AST_META: AST_ID=85 | TYPE=FUNCTION | NAME=sub | COMPLEXITY=5 | LINES=8 */

impl Sub for i32 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self - rhs
    }
}
/* AST_META: AST_ID=86 | TYPE=FUNCTION | NAME=rem | COMPLEXITY=2 | LINES=7 */

#[lang = "rem"]
pub trait Rem<RHS = Self> {
    type Output;

    fn rem(self, rhs: RHS) -> Self::Output;
}
/* AST_META: AST_ID=87 | TYPE=FUNCTION | NAME=rem | COMPLEXITY=5 | LINES=8 */

impl Rem for usize {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        self % rhs
    }
}
/* AST_META: AST_ID=88 | TYPE=FUNCTION | NAME=bitor | COMPLEXITY=2 | LINES=8 */

#[lang = "bitor"]
pub trait BitOr<RHS = Self> {
    type Output;

    #[must_use]
    fn bitor(self, rhs: RHS) -> Self::Output;
}
/* AST_META: AST_ID=89 | TYPE=FUNCTION | NAME=bitor | COMPLEXITY=5 | LINES=8 */

impl BitOr for bool {
    type Output = bool;

    fn bitor(self, rhs: bool) -> bool {
        self | rhs
    }
}
/* AST_META: AST_ID=90 | TYPE=FUNCTION | NAME=bitor | COMPLEXITY=5 | LINES=8 */

impl<'a> BitOr<bool> for &'a bool {
    type Output = bool;

    fn bitor(self, rhs: bool) -> bool {
        *self | rhs
    }
}
/* AST_META: AST_ID=91 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=2 | LINES=6 */

#[lang = "eq"]
pub trait PartialEq<Rhs: ?Sized = Self> {
    fn eq(&self, other: &Rhs) -> bool;
    fn ne(&self, other: &Rhs) -> bool;
}
/* AST_META: AST_ID=92 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=6 | LINES=9 */

impl PartialEq for u8 {
    fn eq(&self, other: &u8) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u8) -> bool {
        (*self) != (*other)
    }
}
/* AST_META: AST_ID=93 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=6 | LINES=9 */

impl PartialEq for u16 {
    fn eq(&self, other: &u16) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u16) -> bool {
        (*self) != (*other)
    }
}
/* AST_META: AST_ID=94 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=6 | LINES=9 */

impl PartialEq for u32 {
    fn eq(&self, other: &u32) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u32) -> bool {
        (*self) != (*other)
    }
}
/* AST_META: AST_ID=95 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=6 | LINES=9 */

impl PartialEq for u64 {
    fn eq(&self, other: &u64) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &u64) -> bool {
        (*self) != (*other)
    }
}
/* AST_META: AST_ID=96 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=6 | LINES=9 */

impl PartialEq for usize {
    fn eq(&self, other: &usize) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &usize) -> bool {
        (*self) != (*other)
    }
}
/* AST_META: AST_ID=97 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=6 | LINES=9 */

impl PartialEq for i8 {
    fn eq(&self, other: &i8) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &i8) -> bool {
        (*self) != (*other)
    }
}
/* AST_META: AST_ID=98 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=6 | LINES=9 */

impl PartialEq for i32 {
    fn eq(&self, other: &i32) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &i32) -> bool {
        (*self) != (*other)
    }
}
/* AST_META: AST_ID=99 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=6 | LINES=9 */

impl PartialEq for isize {
    fn eq(&self, other: &isize) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &isize) -> bool {
        (*self) != (*other)
    }
}
/* AST_META: AST_ID=100 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=6 | LINES=9 */

impl PartialEq for char {
    fn eq(&self, other: &char) -> bool {
        (*self) == (*other)
    }
    fn ne(&self, other: &char) -> bool {
        (*self) != (*other)
    }
}
/* AST_META: AST_ID=101 | TYPE=FUNCTION | NAME=eq | COMPLEXITY=6 | LINES=9 */

impl<T: ?Sized> PartialEq for *const T {
    fn eq(&self, other: &*const T) -> bool {
        *self == *other
    }
    fn ne(&self, other: &*const T) -> bool {
        *self != *other
    }
}
/* AST_META: AST_ID=102 | TYPE=FUNCTION | NAME=neg | COMPLEXITY=2 | LINES=7 */

#[lang = "neg"]
pub trait Neg {
    type Output;

    fn neg(self) -> Self::Output;
}
/* AST_META: AST_ID=103 | TYPE=FUNCTION | NAME=neg | COMPLEXITY=5 | LINES=8 */

impl Neg for i8 {
    type Output = i8;

    fn neg(self) -> i8 {
        -self
    }
}
/* AST_META: AST_ID=104 | TYPE=FUNCTION | NAME=neg | COMPLEXITY=5 | LINES=8 */

impl Neg for i16 {
    type Output = i16;

    fn neg(self) -> i16 {
        self
    }
}
/* AST_META: AST_ID=105 | TYPE=FUNCTION | NAME=neg | COMPLEXITY=5 | LINES=8 */

impl Neg for isize {
    type Output = isize;

    fn neg(self) -> isize {
        -self
    }
}
/* AST_META: AST_ID=106 | TYPE=FUNCTION | NAME=neg | COMPLEXITY=5 | LINES=8 */

impl Neg for f32 {
    type Output = f32;

    fn neg(self) -> f32 {
        -self
    }
}
/* AST_META: AST_ID=107 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=5 */

pub enum Option<T> {
    Some(T),
    None,
}
/* AST_META: AST_ID=108 | TYPE=FUNCTION | NAME=PhantomData | COMPLEXITY=2 | LINES=14 */

pub use Option::*;

#[lang = "phantom_data"]
pub struct PhantomData<T: PointeeSized>;

#[lang = "fn_once"]
#[rustc_paren_sugar]
pub trait FnOnce<Args: Tuple> {
    #[lang = "fn_once_output"]
    type Output;

    extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
}
/* AST_META: AST_ID=109 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[lang = "fn_mut"]
#[rustc_paren_sugar]
pub trait FnMut<Args: Tuple>: FnOnce<Args> {
    extern "rust-call" fn call_mut(&mut self, args: Args) -> Self::Output;
}
/* AST_META: AST_ID=110 | TYPE=FUNCTION | NAME=panic | COMPLEXITY=7 | LINES=9 */

#[lang = "panic"]
#[track_caller]
pub fn panic(_msg: &'static str) -> ! {
    unsafe {
        libc::puts("Panicking\n\0" as *const str as *const u8);
        intrinsics::abort();
    }
}
/* AST_META: AST_ID=111 | TYPE=FUNCTION | NAME=$lang | COMPLEXITY=10 | LINES=16 */

macro_rules! panic_const {
    ($($lang:ident = $message:expr,)+) => {
        pub mod panic_const {
            use super::*;

            $(
                #[track_caller]
                #[lang = stringify!($lang)]
                pub fn $lang() -> ! {
                    panic($message);
                }
            )+
        }
    }
}
/* AST_META: AST_ID=112 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=13 */

panic_const! {
    panic_const_add_overflow = "attempt to add with overflow",
    panic_const_sub_overflow = "attempt to subtract with overflow",
    panic_const_mul_overflow = "attempt to multiply with overflow",
    panic_const_div_overflow = "attempt to divide with overflow",
    panic_const_rem_overflow = "attempt to calculate the remainder with overflow",
    panic_const_neg_overflow = "attempt to negate with overflow",
    panic_const_shr_overflow = "attempt to shift right with overflow",
    panic_const_shl_overflow = "attempt to shift left with overflow",
    panic_const_div_by_zero = "attempt to divide by zero",
    panic_const_rem_by_zero = "attempt to calculate the remainder with a divisor of zero",
}
/* AST_META: AST_ID=113 | TYPE=FUNCTION | NAME=panic_cannot_unwind | COMPLEXITY=7 | LINES=8 */

#[lang = "panic_cannot_unwind"]
fn panic_cannot_unwind() -> ! {
    unsafe {
        libc::puts("Panicking\n\0" as *const str as *const u8);
        intrinsics::abort();
    }
}
/* AST_META: AST_ID=114 | TYPE=FUNCTION | NAME=panic_in_cleanup | COMPLEXITY=7 | LINES=9 */

#[lang = "panic_in_cleanup"]
#[rustc_nounwind]
fn panic_in_cleanup() -> ! {
    unsafe {
        libc::printf("panic in a destructor during cleanup\n\0" as *const str as *const i8);
        intrinsics::abort();
    }
}
/* AST_META: AST_ID=115 | TYPE=FUNCTION | NAME=panic_bounds_check | COMPLEXITY=7 | LINES=13 */

#[lang = "panic_bounds_check"]
#[track_caller]
fn panic_bounds_check(index: usize, len: usize) -> ! {
    unsafe {
        libc::printf(
            "index out of bounds: the len is %d but the index is %d\n\0" as *const str as *const i8,
            len,
            index,
        );
        intrinsics::abort();
    }
}
/* AST_META: AST_ID=116 | TYPE=FUNCTION | NAME=eh_personality | COMPLEXITY=5 | LINES=5 */

#[lang = "eh_personality"]
fn eh_personality() -> ! {
    loop {}
}
/* AST_META: AST_ID=117 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=8 */

#[lang = "drop_in_place"]
#[allow(unconditional_recursion)]
pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
    // Code here does not matter - this is replaced by the
    // real drop glue by the compiler.
    drop_in_place(to_drop);
}
/* AST_META: AST_ID=118 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */

#[lang = "unpin"]
pub auto trait Unpin {}
/* AST_META: AST_ID=119 | TYPE=FUNCTION | NAME=deref | COMPLEXITY=2 | LINES=7 */

#[lang = "deref"]
pub trait Deref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}
/* AST_META: AST_ID=120 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

pub trait Allocator {}
/* AST_META: AST_ID=121 | TYPE=IMPL | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl Allocator for () {}
/* AST_META: AST_ID=122 | TYPE=STRUCT | NAME=Global; | COMPLEXITY=4 | LINES=5 */

#[lang = "global_alloc_ty"]
pub struct Global;

impl Allocator for Global {}
/* AST_META: AST_ID=123 | TYPE=STRUCT | NAME=NonNull | COMPLEXITY=4 | LINES=7 */

#[repr(transparent)]
#[rustc_layout_scalar_valid_range_start(1)]
#[rustc_nonnull_optimization_guaranteed]
pub struct NonNull<T: PointeeSized>(pub *const T);

impl<T: PointeeSized, U: PointeeSized> CoerceUnsized<NonNull<U>> for NonNull<T> where T: Unsize<U> {}
/* AST_META: AST_ID=124 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl<T: PointeeSized, U: PointeeSized> DispatchFromDyn<NonNull<U>> for NonNull<T> where T: Unsize<U> {}
/* AST_META: AST_ID=125 | TYPE=STRUCT | NAME=Unique | COMPLEXITY=2 | LINES=5 */

pub struct Unique<T: PointeeSized> {
    pub pointer: NonNull<T>,
    pub _marker: PhantomData<T>,
}
/* AST_META: AST_ID=126 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl<T: PointeeSized, U: PointeeSized> CoerceUnsized<Unique<U>> for Unique<T> where T: Unsize<U> {}
/* AST_META: AST_ID=127 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl<T: PointeeSized, U: PointeeSized> DispatchFromDyn<Unique<U>> for Unique<T> where T: Unsize<U> {}
/* AST_META: AST_ID=128 | TYPE=STRUCT | NAME=Box | COMPLEXITY=4 | LINES=5 */

#[lang = "owned_box"]
pub struct Box<T: ?Sized, A: Allocator = Global>(Unique<T>, A);

impl<T: ?Sized + Unsize<U>, U: ?Sized, A: Allocator> CoerceUnsized<Box<U, A>> for Box<T, A> {}
/* AST_META: AST_ID=129 | TYPE=FUNCTION | NAME=new | COMPLEXITY=9 | LINES=11 */

impl<T> Box<T> {
    pub fn new(val: T) -> Box<T> {
        unsafe {
            let size = intrinsics::size_of::<T>();
            let ptr = libc::malloc(size);
            intrinsics::copy(&val as *const T as *const u8, ptr, size);
            Box(Unique { pointer: NonNull(ptr as *const T), _marker: PhantomData }, Global)
        }
    }
}
/* AST_META: AST_ID=130 | TYPE=FUNCTION | NAME=drop | COMPLEXITY=10 | LINES=9 */

impl<T: ?Sized, A: Allocator> Drop for Box<T, A> {
    fn drop(&mut self) {
        // inner value is dropped by compiler.
        unsafe {
            libc::free(self.0.pointer.0 as *mut u8);
        }
    }
}
/* AST_META: AST_ID=131 | TYPE=FUNCTION | NAME=deref | COMPLEXITY=5 | LINES=8 */

impl<T: ?Sized, A: Allocator> Deref for Box<T, A> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &**self
    }
}
/* AST_META: AST_ID=132 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=6 | LINES=5 */

#[lang = "exchange_malloc"]
unsafe fn allocate(size: usize, _align: usize) -> *mut u8 {
    libc::malloc(size)
}
/* AST_META: AST_ID=133 | TYPE=FUNCTION | NAME=drop | COMPLEXITY=2 | LINES=5 */

#[lang = "drop"]
pub trait Drop {
    fn drop(&mut self);
}
/* AST_META: AST_ID=134 | TYPE=STRUCT | NAME=ManuallyDrop | COMPLEXITY=2 | LINES=6 */

#[lang = "manually_drop"]
#[repr(transparent)]
pub struct ManuallyDrop<T: ?Sized> {
    pub value: T,
}
/* AST_META: AST_ID=135 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

#[lang = "maybe_uninit"]
#[repr(transparent)]
pub union MaybeUninit<T> {
    pub uninit: (),
    pub value: ManuallyDrop<T>,
}
/* AST_META: AST_ID=136 | TYPE=FUNCTION | NAME=abort | COMPLEXITY=32 | LINES=31 */

pub mod intrinsics {
    #[rustc_intrinsic]
    pub const fn black_box<T>(_dummy: T) -> T;
    #[rustc_intrinsic]
    pub fn abort() -> !;
    #[rustc_intrinsic]
    pub fn size_of<T>() -> usize;
    #[rustc_intrinsic]
    pub unsafe fn size_of_val<T: ?::Sized>(val: *const T) -> usize;
    #[rustc_intrinsic]
    pub fn align_of<T>() -> usize;
    #[rustc_intrinsic]
    pub unsafe fn align_of_val<T: ?::Sized>(val: *const T) -> usize;
    #[rustc_intrinsic]
    pub unsafe fn copy<T>(src: *const T, dst: *mut T, count: usize);
    #[rustc_intrinsic]
    pub unsafe fn transmute<T, U>(e: T) -> U;
    #[rustc_intrinsic]
    pub unsafe fn ctlz_nonzero<T>(x: T) -> u32;
    #[rustc_intrinsic]
    pub fn needs_drop<T: ?::Sized>() -> bool;
    #[rustc_intrinsic]
    pub fn bitreverse<T>(x: T) -> T;
    #[rustc_intrinsic]
    pub fn bswap<T>(x: T) -> T;
    #[rustc_intrinsic]
    pub unsafe fn write_bytes<T>(dst: *mut T, val: u8, count: usize);
    #[rustc_intrinsic]
    pub unsafe fn unreachable() -> !;
}
/* AST_META: AST_ID=137 | TYPE=FUNCTION | NAME=puts | COMPLEXITY=4 | LINES=17 */

pub mod libc {
    #[link(name = "c")]
    extern "C" {
        pub fn puts(s: *const u8) -> i32;
        pub fn printf(format: *const i8, ...) -> i32;
        pub fn malloc(size: usize) -> *mut u8;
        pub fn free(ptr: *mut u8);
        pub fn memcpy(dst: *mut u8, src: *const u8, size: usize);
        pub fn memmove(dst: *mut u8, src: *const u8, size: usize);
        pub fn strncpy(dst: *mut u8, src: *const u8, size: usize);
        pub fn fflush(stream: *mut i32) -> i32;
        pub fn exit(status: i32);

        pub static stdout: *mut i32;
    }
}
/* AST_META: AST_ID=138 | TYPE=FUNCTION | NAME=index | COMPLEXITY=2 | LINES=6 */

#[lang = "index"]
pub trait Index<Idx: ?Sized> {
    type Output: ?Sized;
    fn index(&self, index: Idx) -> &Self::Output;
}
/* AST_META: AST_ID=139 | TYPE=FUNCTION | NAME=index | COMPLEXITY=5 | LINES=8 */

impl<T> Index<usize> for [T; 3] {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self[index]
    }
}
/* AST_META: AST_ID=140 | TYPE=FUNCTION | NAME=index | COMPLEXITY=5 | LINES=8 */

impl<T> Index<usize> for [T] {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self[index]
    }
}
/* AST_META: AST_ID=141 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */

extern "C" {
    type VaListImpl;
}
/* AST_META: AST_ID=142 | TYPE=STRUCT | NAME=VaList | COMPLEXITY=2 | LINES=10 */

#[lang = "va_list"]
#[repr(transparent)]
pub struct VaList<'a>(&'a mut VaListImpl);

#[rustc_builtin_macro]
#[rustc_macro_transparency = "semitransparent"]
pub macro stringify($($t:tt)*) {
    /* compiler built-in */
}
/* AST_META: AST_ID=143 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[rustc_builtin_macro]
#[rustc_macro_transparency = "semitransparent"]
pub macro file() {
    /* compiler built-in */
}
/* AST_META: AST_ID=144 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[rustc_builtin_macro]
#[rustc_macro_transparency = "semitransparent"]
pub macro line() {
    /* compiler built-in */
}
/* AST_META: AST_ID=145 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[rustc_builtin_macro]
#[rustc_macro_transparency = "semitransparent"]
pub macro cfg() {
    /* compiler built-in */
}
/* AST_META: AST_ID=146 | TYPE=STRUCT | NAME=PanicLocation | COMPLEXITY=2 | LINES=9 */

pub static A_STATIC: u8 = 42;

#[lang = "panic_location"]
struct PanicLocation {
    file: &'static str,
    line: u32,
    column: u32,
}
/* AST_META: AST_ID=147 | TYPE=FUNCTION | NAME=get_tls | COMPLEXITY=2 | LINES=8 */

#[unsafe(no_mangle)]
pub fn get_tls() -> u8 {
    #[thread_local]
    static A: u8 = 42;

    A
}
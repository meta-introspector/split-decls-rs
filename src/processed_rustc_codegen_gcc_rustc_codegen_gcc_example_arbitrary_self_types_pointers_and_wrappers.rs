// SRC: ../rust/compiler/rustc_codegen_gcc/example/arbitrary_self_types_pointers_and_wrappers.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=3 | LINES=10 */
// Adapted from rustc run-pass test suite

#[feature(arbitrary_self_types, unsize, coerce_unsized, dispatch_from_dyn)]
#[feature(rustc_attrs)]
#[allow(internal_features)]

use std::{
    ops::{Deref, CoerceUnsized, DispatchFromDyn},
    marker::Unsize,
};
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=Ptr | COMPLEXITY=5 | LINES=10 */

struct Ptr<T: ?Sized>(Box<T>);

impl<T: ?Sized> Deref for Ptr<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &*self.0
    }
}
/* AST_META: AST_ID=3 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl<T: Unsize<U> + ?Sized, U: ?Sized> CoerceUnsized<Ptr<U>> for Ptr<T> {}
/* AST_META: AST_ID=4 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl<T: Unsize<U> + ?Sized, U: ?Sized> DispatchFromDyn<Ptr<U>> for Ptr<T> {}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=Wrapper | COMPLEXITY=5 | LINES=10 */

struct Wrapper<T: ?Sized>(T);

impl<T: ?Sized> Deref for Wrapper<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
/* AST_META: AST_ID=6 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=2 */

impl<T: CoerceUnsized<U>, U> CoerceUnsized<Wrapper<U>> for Wrapper<T> {}
/* AST_META: AST_ID=7 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=1 */
impl<T: DispatchFromDyn<U>, U> DispatchFromDyn<Wrapper<U>> for Wrapper<T> {}
/* AST_META: AST_ID=8 | TYPE=FUNCTION | NAME=ptr_wrapper | COMPLEXITY=2 | LINES=7 */


trait Trait {
    fn ptr_wrapper(self: Ptr<Wrapper<Self>>) -> i32;
    fn wrapper_ptr(self: Wrapper<Ptr<Self>>) -> i32;
    fn wrapper_ptr_wrapper(self: Wrapper<Ptr<Wrapper<Self>>>) -> i32;
}
/* AST_META: AST_ID=9 | TYPE=FUNCTION | NAME=ptr_wrapper | COMPLEXITY=7 | LINES=12 */

impl Trait for i32 {
    fn ptr_wrapper(self: Ptr<Wrapper<Self>>) -> i32 {
        **self
    }
    fn wrapper_ptr(self: Wrapper<Ptr<Self>>) -> i32 {
        **self
    }
    fn wrapper_ptr_wrapper(self: Wrapper<Ptr<Wrapper<Self>>>) -> i32 {
        ***self
    }
}
/* AST_META: AST_ID=10 | TYPE=FUNCTION | NAME=main | COMPLEXITY=2 | LINES=11 */

fn main() {
    let pw = Ptr(Box::new(Wrapper(5))) as Ptr<Wrapper<dyn Trait>>;
    assert_eq!(pw.ptr_wrapper(), 5);

    let wp = Wrapper(Ptr(Box::new(6))) as Wrapper<Ptr<dyn Trait>>;
    assert_eq!(wp.wrapper_ptr(), 6);

    let wpw = Wrapper(Ptr(Box::new(Wrapper(7)))) as Wrapper<Ptr<Wrapper<dyn Trait>>>;
    assert_eq!(wpw.wrapper_ptr_wrapper(), 7);
}
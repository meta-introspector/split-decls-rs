// SRC: ../rust/compiler/rustc_data_structures/src/frozen.rs
/* AST_META: AST_ID=1 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=6 | LINES=7 */
// An immutable, owned value (except for interior mutability).
//
// The purpose of `Frozen` is to make a value immutable for the sake of defensive programming. For example,
// suppose we have the following:
//
// ```rust
// struct Bar { /* some data */ }
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */
//
// struct Foo {
//     /// Some computed data that should never change after construction.
//     pub computed: Bar,
//
//     /* some other fields */
// }
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=UNNAMED | COMPLEXITY=3 | LINES=5 */
//
// impl Bar {
//     /// Mutate the `Bar`.
//     pub fn mutate(&mut self) { }
// }
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */
// ```
//
// Now suppose we want to pass around a mutable `Foo` instance but, we want to make sure that
// `computed` does not change accidentally (e.g. somebody might accidentally call
// `foo.computed.mutate()`). This is what `Frozen` is for. We can do the following:
//
// ```
// # struct Bar {}
/* AST_META: AST_ID=5 | TYPE=STRUCT | NAME=UNNAMED | COMPLEXITY=2 | LINES=8 */
// use crate::rustc_data_structures::frozen::Frozen;
//
// struct Foo {
//     /// Some computed data that should never change after construction.
//     pub computed: Frozen<Bar>,
//
//     /* some other fields */
// }
/* AST_META: AST_ID=6 | TYPE=FUNCTION | NAME=Frozen | COMPLEXITY=4 | LINES=21 */
// ```
//
// `Frozen` impls `Deref`, so we can ergonomically call methods on `Bar`, but it doesn't `impl
// DerefMut`. Now calling `foo.compute.mutate()` will result in a compile-time error stating that
// `mutate` requires a mutable reference but we don't have one.
//
// # Caveats
//
// - `Frozen` doesn't try to defend against interior mutability (e.g. `Frozen<RefCell<Bar>>`).
// - `Frozen` doesn't pin it's contents (e.g. one could still do `foo.computed =
//    Frozen::freeze(new_bar)`).

/// An owned immutable value.
#[derive(Debug, Clone)]
pub struct Frozen<T>(T);

impl<T> Frozen<T> {
    pub fn freeze(val: T) -> Self {
        Frozen(val)
    }
}
/* AST_META: AST_ID=7 | TYPE=FUNCTION | NAME=deref | COMPLEXITY=5 | LINES=8 */

impl<T> std::ops::Deref for Frozen<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
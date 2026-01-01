// SRC: ../rust/compiler/rustc_type_ir/src/upcast.rs
/* AST_META: AST_ID=1 | TYPE=FUNCTION | NAME=upcast | COMPLEXITY=2 | LINES=4 */
/// An `Into`-like trait that takes `TyCtxt` to perform interner-specific transformations.
pub trait Upcast<I, T> {
    fn upcast(self, interner: I) -> T;
}
/* AST_META: AST_ID=2 | TYPE=FUNCTION | NAME=upcast | COMPLEXITY=5 | LINES=9 */

impl<I, T, U> Upcast<I, U> for T
where
    U: UpcastFrom<I, T>,
{
    fn upcast(self, interner: I) -> U {
        U::upcast_from(self, interner)
    }
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=upcast_from | COMPLEXITY=2 | LINES=5 */

/// A `From`-like trait that takes `TyCtxt` to perform interner-specific transformations.
pub trait UpcastFrom<I, T> {
    fn upcast_from(from: T, interner: I) -> Self;
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=upcast_from | COMPLEXITY=5 | LINES=6 */

impl<I, T> UpcastFrom<I, T> for T {
    fn upcast_from(from: T, _tcx: I) -> Self {
        from
    }
}
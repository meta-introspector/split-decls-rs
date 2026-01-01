// SRC: ../rust/compiler/rustc_type_ir/src/lang_items.rs
/* AST_META: AST_ID=1 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=15 */
/// Lang items used by the new trait solver. This can be mapped to whatever internal
/// representation of `LangItem`s used in the underlying compiler implementation.
pub enum SolverLangItem {
    // tidy-alphabetical-start
    AsyncFnKindUpvars,
    AsyncFnOnceOutput,
    CallOnceFuture,
    CallRefFuture,
    CoroutineReturn,
    CoroutineYield,
    DynMetadata,
    FutureOutput,
    Metadata,
    // tidy-alphabetical-end
}
/* AST_META: AST_ID=2 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=7 */

pub enum SolverAdtLangItem {
    // tidy-alphabetical-start
    Option,
    Poll,
    // tidy-alphabetical-end
}
/* AST_META: AST_ID=3 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=3 | LINES=33 */

pub enum SolverTraitLangItem {
    // tidy-alphabetical-start
    AsyncFn,
    AsyncFnKindHelper,
    AsyncFnMut,
    AsyncFnOnce,
    AsyncFnOnceOutput,
    AsyncIterator,
    BikeshedGuaranteedNoDrop,
    Clone,
    Copy,
    Coroutine,
    Destruct,
    DiscriminantKind,
    Drop,
    Fn,
    FnMut,
    FnOnce,
    FnPtrTrait,
    FusedIterator,
    Future,
    Iterator,
    MetaSized,
    PointeeSized,
    PointeeTrait,
    Sized,
    TransmuteTrait,
    Tuple,
    Unpin,
    Unsize,
    // tidy-alphabetical-end
}
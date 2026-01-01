// SRC: ../rust/compiler/rustc_mir_dataflow/src/impls/mod.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

pub use self::borrowed_locals::{MaybeBorrowedLocals, borrowed_locals};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
pub use self::initialized::{
    EverInitializedPlaces, EverInitializedPlacesDomain, MaybeInitializedPlaces,
    MaybeUninitializedPlaces, MaybeUninitializedPlacesDomain,
};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
pub use self::liveness::{
    DefUse, MaybeLiveLocals, MaybeTransitiveLiveLocals,
    TransferFunction as LivenessTransferFunction,
};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
pub use self::storage_liveness::{
    MaybeRequiresStorage, MaybeStorageDead, MaybeStorageLive, always_storage_live_locals,
};
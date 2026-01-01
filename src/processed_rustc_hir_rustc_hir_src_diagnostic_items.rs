// SRC: ../rust/compiler/rustc_hir/src/diagnostic_items.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use crate::rustc_data_structures::fx::FxIndexMap;
use crate::rustc_data_structures::stable_hasher::{HashStable, StableHasher};
/* AST_META: AST_ID=2 | TYPE=STRUCT | NAME=DiagnosticItems | COMPLEXITY=2 | LINES=10 */
use crate::rustc_complete::Symbol;
use crate::rustc_complete::def_id::DefIdMap;

use crate::def_id::DefId;

#[derive(Debug, Default)]
pub struct DiagnosticItems {
    pub id_to_name: DefIdMap<Symbol>,
    pub name_to_id: FxIndexMap<Symbol, DefId>,
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=hash_stable | COMPLEXITY=5 | LINES=7 */

impl<CTX: crate::HashStableContext> HashStable<CTX> for DiagnosticItems {
    #[inline]
    fn hash_stable(&self, ctx: &mut CTX, hasher: &mut StableHasher) {
        self.name_to_id.hash_stable(ctx, hasher);
    }
}
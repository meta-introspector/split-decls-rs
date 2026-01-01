// SRC: ../rust/compiler/rustc_query_system/src/ich/mod.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=3 */
// ICH - Incremental Compilation Hash

use crate::rustc_complete::{Symbol, sym};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=1 | LINES=16 */

pub use self::hcx::StableHashingContext;


pub const IGNORED_ATTRIBUTES: &[Symbol] = &[
    sym::cfg_trace, // FIXME should this really be ignored?
    sym::rustc_if_this_changed,
    sym::rustc_then_this_would_need,
    sym::rustc_dirty,
    sym::rustc_clean,
    sym::rustc_partition_reused,
    sym::rustc_partition_codegened,
    sym::rustc_expected_cgu_reuse,
];
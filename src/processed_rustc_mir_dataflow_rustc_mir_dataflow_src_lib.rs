// SRC: ../rust/compiler/rustc_mir_dataflow/src/lib.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=3 | LINES=18 */
// tidy-alphabetical-start
#[feature(assert_matches)]
#[feature(associated_type_defaults)]
#[feature(box_patterns)]
#[feature(exact_size_is_empty)]
#[feature(file_buffered)]
#[feature(never_type)]
#[feature(try_blocks)]
// tidy-alphabetical-end

use crate::rustc_complete::ty;

// Please change the public `use` directives cautiously, as they might be used by external tools.
// See issue #120130.
pub use self::drop_flag_effects::{
    DropFlagState, drop_flag_effects_for_function_entry, drop_flag_effects_for_location,
    move_path_children_matching, on_all_children_bits, on_lookup_result_bits,
};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
pub use self::framework::{
    Analysis, Backward, Direction, Forward, GenKill, JoinSemiLattice, MaybeReachable, Results,
    ResultsCursor, ResultsVisitor, fmt, graphviz, lattice, visit_reachable_results, visit_results,
};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=14 */
use self::move_paths::MoveData;


rustc_fluent_macro::fluent_messages! { "../messages.ftl" }
/* AST_META: AST_ID=4 | TYPE=STRUCT | NAME=MoveDataTypingEnv | COMPLEXITY=2 | LINES=5 */

pub struct MoveDataTypingEnv<'tcx> {
    pub move_data: MoveData<'tcx>,
    pub typing_env: ty::TypingEnv<'tcx>,
}
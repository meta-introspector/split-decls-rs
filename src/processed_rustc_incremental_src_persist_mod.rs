// SRC: ../rust/compiler/rustc_incremental/src/persist/mod.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=13 */
// When in incremental mode, this pass dumps out the dependency graph
// into the given directory. At the same time, it also hashes the
// various HIR nodes.


pub use fs::{finalize_session_directory, in_incr_comp_dir, in_incr_comp_dir_sess};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
pub use load::{LoadResult, load_query_result_cache, setup_dep_graph};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=1 | LINES=3 */
pub(crate) use save::save_dep_graph;
pub use save::save_work_product_index;
pub use work_product::copy_cgu_workproduct_to_incr_comp_cache_dir;
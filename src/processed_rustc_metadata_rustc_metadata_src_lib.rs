// SRC: ../rust/compiler/rustc_metadata/src/lib.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=3 | LINES=30 */
// tidy-alphabetical-start
#[allow(internal_features)]
#[doc(html_root_url = "https://doc.rust-lang.org/nightly/nightly-rustc/")]
#[doc(rust_logo)]
#[feature(decl_macro)]
#[feature(error_iter)]
#[feature(file_buffered)]
#[feature(gen_blocks)]
#[feature(if_let_guard)]
#[feature(macro_metavar_expr)]
#[feature(min_specialization)]
#[feature(never_type)]
#[feature(proc_macro_internals)]
#[feature(rustdoc_internals)]
#[feature(trusted_len)]
// tidy-alphabetical-end

pub use rmeta::provide;



pub use creader::{DylibError, load_symbol_from_dylib};
/* AST_META: AST_ID=2 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
pub use fs::{METADATA_FILENAME, emit_wrapper_file};
/* AST_META: AST_ID=3 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=4 */
pub use native_libs::{
    NativeLibSearchFallback, find_native_static_library, try_find_native_dynamic_library,
    try_find_native_static_library, walk_native_lib_search_dirs,
};
/* AST_META: AST_ID=4 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=1 */
pub use rmeta::{EncodedMetadata, METADATA_HEADER, encode_metadata, rendered_const};
/* AST_META: AST_ID=5 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */

rustc_fluent_macro::fluent_messages! { "../messages.ftl" }
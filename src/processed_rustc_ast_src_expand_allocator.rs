// SRC: ../rust/compiler/rustc_ast/src/expand/allocator.rs
/* AST_META: AST_ID=1 | TYPE=USE | NAME=UNNAMED | COMPLEXITY=2 | LINES=2 */
use rustc_macros::HashStable_Generic;
use crate::rustc_complete::{Symbol, sym};
/* AST_META: AST_ID=2 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=6 */

#[derive(Clone, Debug, Copy, Eq, PartialEq, HashStable_Generic)]
pub enum AllocatorKind {
    Global,
    Default,
}
/* AST_META: AST_ID=3 | TYPE=FUNCTION | NAME=global_fn_name | COMPLEXITY=3 | LINES=4 */

pub fn global_fn_name(base: Symbol) -> String {
    format!("__rust_{base}")
}
/* AST_META: AST_ID=4 | TYPE=FUNCTION | NAME=default_fn_name | COMPLEXITY=3 | LINES=4 */

pub fn default_fn_name(base: Symbol) -> String {
    format!("__rdl_{base}")
}
/* AST_META: AST_ID=5 | TYPE=FUNCTION | NAME=alloc_error_handler_name | COMPLEXITY=6 | LINES=7 */

pub fn alloc_error_handler_name(alloc_error_handler_kind: AllocatorKind) -> &'static str {
    match alloc_error_handler_kind {
        AllocatorKind::Global => "__rg_oom",
        AllocatorKind::Default => "__rdl_oom",
    }
}
/* AST_META: AST_ID=6 | TYPE=ENUM | NAME=UNNAMED | COMPLEXITY=2 | LINES=10 */

pub const NO_ALLOC_SHIM_IS_UNSTABLE: &str = "__rust_no_alloc_shim_is_unstable_v2";

pub enum AllocatorTy {
    Layout,
    Ptr,
    ResultPtr,
    Unit,
    Usize,
}
/* AST_META: AST_ID=7 | TYPE=STRUCT | NAME=AllocatorMethod | COMPLEXITY=2 | LINES=6 */

pub struct AllocatorMethod {
    pub name: Symbol,
    pub inputs: &'static [AllocatorMethodInput],
    pub output: AllocatorTy,
}
/* AST_META: AST_ID=8 | TYPE=STRUCT | NAME=AllocatorMethodInput | COMPLEXITY=2 | LINES=5 */

pub struct AllocatorMethodInput {
    pub name: &'static str,
    pub ty: AllocatorTy,
}
/* AST_META: AST_ID=9 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=7 */

pub static ALLOCATOR_METHODS: &[AllocatorMethod] = &[
    AllocatorMethod {
        name: sym::alloc,
        inputs: &[AllocatorMethodInput { name: "layout", ty: AllocatorTy::Layout }],
        output: AllocatorTy::ResultPtr,
    },
/* AST_META: AST_ID=10 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=4 | LINES=8 */
    AllocatorMethod {
        name: sym::dealloc,
        inputs: &[
            AllocatorMethodInput { name: "ptr", ty: AllocatorTy::Ptr },
            AllocatorMethodInput { name: "layout", ty: AllocatorTy::Layout },
        ],
        output: AllocatorTy::Unit,
    },
/* AST_META: AST_ID=11 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=5 | LINES=9 */
    AllocatorMethod {
        name: sym::realloc,
        inputs: &[
            AllocatorMethodInput { name: "ptr", ty: AllocatorTy::Ptr },
            AllocatorMethodInput { name: "layout", ty: AllocatorTy::Layout },
            AllocatorMethodInput { name: "new_size", ty: AllocatorTy::Usize },
        ],
        output: AllocatorTy::ResultPtr,
    },
/* AST_META: AST_ID=12 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=3 | LINES=5 */
    AllocatorMethod {
        name: sym::alloc_zeroed,
        inputs: &[AllocatorMethodInput { name: "layout", ty: AllocatorTy::Layout }],
        output: AllocatorTy::ResultPtr,
    },
/* AST_META: AST_ID=13 | TYPE=BLOCK | NAME=UNNAMED | COMPLEXITY=1 | LINES=1 */
];
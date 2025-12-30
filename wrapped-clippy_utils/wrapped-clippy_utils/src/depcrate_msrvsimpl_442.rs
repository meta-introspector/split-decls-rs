// Generated macro for impl_442 (impl)
macro_rules! Depcrate_msrvsimpl_442 {
() => {
// Module: crate::msrvs
// Provides: {"impl_442"}
// Dependencies: {}
impl Msrv { # [doc = " Returns the MSRV at the current node"] # [doc = ""] # [doc = " If the crate being linted uses an `#[clippy::msrv]` attribute this will search the parent"] # [doc = " nodes for that attribute, prefer to run this check after cheaper pattern matching operations"] pub fn current (self , cx : & LateContext < '_ >) -> Option < RustcVersion > { if SEEN_MSRV_ATTR . load (Ordering :: Relaxed) { let start = cx . last_node_with_lint_attrs ; if let Some (msrv_attr) = once (start) . chain (cx . tcx . hir_parent_id_iter (start)) . find_map (| id | parse_attrs (cx . tcx . sess , cx . tcx . hir_attrs (id))) { return Some (msrv_attr) ; } } self . 0 } # [doc = " Checks if a required version from [this module](self) is met at the current node"] # [doc = ""] # [doc = " If the crate being linted uses an `#[clippy::msrv]` attribute this will search the parent"] # [doc = " nodes for that attribute, prefer to run this check after cheaper pattern matching operations"] pub fn meets (self , cx : & LateContext < '_ > , required : RustcVersion) -> bool { self . current (cx) . is_none_or (| msrv | msrv >= required) } pub fn read_cargo (& mut self , sess : & Session) { let cargo_msrv = std :: env :: var ("CARGO_PKG_RUST_VERSION") . ok () . and_then (| v | parse_version (Symbol :: intern (& v))) ; match (self . 0 , cargo_msrv) { (None , Some (cargo_msrv)) => self . 0 = Some (cargo_msrv) , (Some (clippy_msrv) , Some (cargo_msrv)) => { if clippy_msrv != cargo_msrv { sess . dcx () . warn (format ! ("the MSRV in `clippy.toml` and `Cargo.toml` differ; using `{clippy_msrv}` from `clippy.toml`")) ; } } , _ => { } , } } }
};
}

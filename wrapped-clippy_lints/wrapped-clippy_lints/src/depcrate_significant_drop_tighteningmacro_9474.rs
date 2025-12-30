// Generated macro for macro_9474 (macro)
macro_rules! Depcrate_significant_drop_tighteningmacro_9474 {
() => {
// Module: crate::significant_drop_tightening
// Provides: {"macro_9474"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Searches for elements marked with `#[clippy::has_significant_drop]` that could be early"] # [doc = " dropped but are in fact dropped at the end of their scopes. In other words, enforces the"] # [doc = " \"tightening\" of their possible lifetimes."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " Elements marked with `#[clippy::has_significant_drop]` are generally synchronizing"] # [doc = " primitives that manage shared resources, as such, it is desired to release them as soon as"] # [doc = " possible to avoid unnecessary resource contention."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " fn main() {"] # [doc = "   let lock = some_sync_resource.lock();"] # [doc = "   let owned_rslt = lock.do_stuff_with_resource();"] # [doc = "   // Only `owned_rslt` is needed but `lock` is still held."] # [doc = "   do_heavy_computation_that_takes_time(owned_rslt);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " fn main() {"] # [doc = "     let owned_rslt = some_sync_resource.lock().do_stuff_with_resource();"] # [doc = "     do_heavy_computation_that_takes_time(owned_rslt);"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.69.0"] pub SIGNIFICANT_DROP_TIGHTENING , nursery , "Searches for elements marked with `#[clippy::has_significant_drop]` that could be early dropped but are in fact dropped at the end of their scopes" }
};
}

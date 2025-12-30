// Generated macro for Box (struct)
macro_rules! Depcrate_boxedBox {
() => {
// Module: crate::boxed
// Provides: {"Box"}
// Dependencies: {}
# [doc = " A pointer type for heap allocation."] # [doc = ""] # [doc = " See the [module-level documentation](../../std/boxed/index.html) for more."] pub struct Box < T : ? Sized , A : Allocator = Global > (Unique < T > , A) ;
};
}

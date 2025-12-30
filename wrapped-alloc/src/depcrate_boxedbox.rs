// Generated macro for Box (struct)
macro_rules! Depcrate_boxedBox {
() => {
// Module: crate::boxed
// Provides: {"Box"}
// Dependencies: {}
# [doc = " A pointer type that uniquely owns a heap allocation of type `T`."] # [doc = ""] # [doc = " See the [module-level documentation](../../std/boxed/index.html) for more."] # [lang = "owned_box"] # [fundamental] # [stable (feature = "rust1" , since = "1.0.0")] # [rustc_insignificant_dtor] # [doc (search_unbox)] pub struct Box < T : ? Sized , # [unstable (feature = "allocator_api" , issue = "32838")] A : Allocator = Global , > (Unique < T > , A) ;
};
}

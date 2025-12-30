// Generated macro for Global (struct)
macro_rules! Depcrate_allocGlobal {
() => {
// Module: crate::alloc
// Provides: {"Global"}
// Dependencies: {}
# [doc = " The global memory allocator."] # [doc = ""] # [doc = " This type implements the [`Allocator`] trait by forwarding calls"] # [doc = " to the allocator registered with the `#[global_allocator]` attribute"] # [doc = " if there is one, or the `std` crate’s default."] # [doc = ""] # [doc = " Note: while this type is unstable, the functionality it provides can be"] # [doc = " accessed through the [free functions in `alloc`](self#functions)."] # [unstable (feature = "allocator_api" , issue = "32838")] # [derive (Copy , Clone , Default , Debug)] # [lang = "global_alloc_ty"] pub struct Global ;
};
}

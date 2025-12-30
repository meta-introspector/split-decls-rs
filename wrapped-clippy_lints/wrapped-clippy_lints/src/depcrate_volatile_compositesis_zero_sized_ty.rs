// Generated macro for is_zero_sized_ty (function)
macro_rules! Depcrate_volatile_compositesis_zero_sized_ty {
() => {
// Module: crate::volatile_composites
// Provides: {"is_zero_sized_ty"}
// Dependencies: {}
# [doc = " Zero-sized types are intrinsically safe to use volatile on since they won't"] # [doc = " actually generate *any* loads or stores. But this is also used to skip zero-sized"] # [doc = " fields of `#[repr(transparent)]` structures."] fn is_zero_sized_ty < 'tcx > (cx : & LateContext < 'tcx > , ty : Ty < 'tcx >) -> bool { cx . layout_of (ty) . is_ok_and (| layout | layout . is_zst ()) }
};
}

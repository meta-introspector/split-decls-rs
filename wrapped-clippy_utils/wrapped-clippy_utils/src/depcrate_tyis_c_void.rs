// Generated macro for is_c_void (function)
macro_rules! Depcrate_tyis_c_void {
() => {
// Module: crate::ty
// Provides: {"is_c_void"}
// Dependencies: {}
# [doc = " Check if the given type is either `core::ffi::c_void`, `std::os::raw::c_void`, or one of the"] # [doc = " platform specific `libc::<platform>::c_void` types in libc."] pub fn is_c_void (cx : & LateContext < '_ > , ty : Ty < '_ >) -> bool { if let ty :: Adt (adt , _) = ty . kind () && let & [krate , .. , name] = & * cx . get_def_path (adt . did ()) && let sym :: libc | sym :: core | sym :: std = krate && name == sym :: c_void { true } else { false } }
};
}

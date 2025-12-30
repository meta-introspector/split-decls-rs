// Generated macro for __pin_project_make_unsafe_drop_in_place_guard (macro)
macro_rules! Depcrate__pin_project_make_unsafe_drop_in_place_guard {
() => {
// Module: crate
// Provides: {"__pin_project_make_unsafe_drop_in_place_guard"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __pin_project_make_unsafe_drop_in_place_guard { (# [pin] $ field : ident) => { $ crate :: __private :: UnsafeDropInPlaceGuard :: new ($ field) } ; ($ field : ident) => { () } ; }
};
}

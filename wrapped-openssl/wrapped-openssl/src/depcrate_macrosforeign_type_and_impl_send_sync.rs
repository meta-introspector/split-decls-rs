// Generated macro for foreign_type_and_impl_send_sync (macro)
macro_rules! Depcrate_macrosforeign_type_and_impl_send_sync {
() => {
// Module: crate::macros
// Provides: {"foreign_type_and_impl_send_sync"}
// Dependencies: {}
macro_rules ! foreign_type_and_impl_send_sync { ($ (# [$ impl_attr : meta]) * type CType = $ ctype : ty ; fn drop = $ drop : expr ; $ (fn clone = $ clone : expr ;) * $ (# [$ owned_attr : meta]) * pub struct $ owned : ident ; $ (# [$ borrowed_attr : meta]) * pub struct $ borrowed : ident ;) => { :: foreign_types :: foreign_type ! { $ (# [$ impl_attr]) * type CType = $ ctype ; fn drop = $ drop ; $ (fn clone = $ clone ;) * $ (# [$ owned_attr]) * pub struct $ owned ; $ (# [$ borrowed_attr]) * pub struct $ borrowed ; } unsafe impl Send for $ owned { } unsafe impl Send for $ borrowed { } unsafe impl Sync for $ owned { } unsafe impl Sync for $ borrowed { } } ; }
};
}

// Generated macro for cenum (macro)
macro_rules! Depcratecenum {
() => {
// Module: crate
// Provides: {"cenum"}
// Dependencies: {}
# [doc = " Defines a C enum as a series of constants."] macro_rules ! cenum { (# [repr ($ ty : ty)] $ (# [$ meta : meta]) * enum $ name : ident { $ ($ (# [$ vmeta : meta]) * const $ variant : ident = $ value : expr) , +, }) => (pub type $ name = $ ty ; $ ($ (# [$ vmeta]) * pub const $ variant : $ name = $ value ;) +) ; (# [repr ($ ty : ty)] $ (# [$ meta : meta]) * enum $ name : ident { $ ($ (# [$ vmeta : meta]) * const $ variant : ident = $ value : expr) ; +; }) => (pub type $ name = $ ty ; $ ($ (# [$ vmeta]) * pub const $ variant : $ name = $ value ;) +) ; ($ (# [$ meta : meta]) * enum $ name : ident { $ ($ (# [$ vmeta : meta]) * const $ variant : ident = $ value : expr) , +, }) => (pub type $ name = c_int ; $ ($ (# [$ vmeta]) * pub const $ variant : $ name = $ value ;) +) ; ($ (# [$ meta : meta]) * enum $ name : ident { $ ($ (# [$ vmeta : meta]) * const $ variant : ident = $ value : expr) ; +; }) => (pub type $ name = c_int ; $ ($ (# [$ vmeta]) * pub const $ variant : $ name = $ value ;) +) ; }
};
}

// Generated macro for define_interface (macro)
macro_rules! Depcrate_impdefine_interface {
() => {
// Module: crate::imp
// Provides: {"define_interface"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! define_interface { ($ name : ident , $ vtbl : ident , $ iid : literal) => { # [repr (transparent)] # [derive (:: core :: cmp :: PartialEq , :: core :: cmp :: Eq , :: core :: clone :: Clone)] pub struct $ name (:: windows_core :: IUnknown) ; unsafe impl :: windows_core :: Interface for $ name { type Vtable = $ vtbl ; const IID : :: windows_core :: GUID = :: windows_core :: GUID :: from_u128 ($ iid) ; } impl :: core :: fmt :: Debug for $ name { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_tuple (stringify ! ($ name)) . field (&:: windows_core :: Interface :: as_raw (self)) . finish () } } } ; ($ name : ident , $ vtbl : ident) => { # [repr (transparent)] # [derive (:: core :: cmp :: PartialEq , :: core :: cmp :: Eq , :: core :: clone :: Clone)] pub struct $ name (:: core :: ptr :: NonNull <:: core :: ffi :: c_void >) ; unsafe impl :: windows_core :: Interface for $ name { type Vtable = $ vtbl ; const IID : :: windows_core :: GUID = :: windows_core :: GUID :: zeroed () ; const UNKNOWN : bool = false ; } impl :: core :: fmt :: Debug for $ name { fn fmt (& self , f : & mut :: core :: fmt :: Formatter) -> core :: fmt :: Result { f . debug_tuple (stringify ! ($ name)) . field (& self . 0) . finish () } } } ; }
};
}

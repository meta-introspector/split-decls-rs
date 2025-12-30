// Generated macro for impl_borrow_decode (macro)
macro_rules! Depcrate_deimpl_borrow_decode {
() => {
// Module: crate::de
// Provides: {"impl_borrow_decode"}
// Dependencies: {}
# [doc = " Helper macro to implement `BorrowDecode` for any type that implements `Decode`."] # [macro_export] macro_rules ! impl_borrow_decode { ($ ty : ty $ (, $ param : tt) *) => { impl <'de $ (, $ param) *, __Context > $ crate :: BorrowDecode <'de , __Context > for $ ty { fn borrow_decode < D : $ crate :: de :: BorrowDecoder <'de , Context = __Context >> (decoder : & mut D ,) -> core :: result :: Result < Self , $ crate :: error :: DecodeError > { $ crate :: Decode :: decode (decoder) } } } ; }
};
}

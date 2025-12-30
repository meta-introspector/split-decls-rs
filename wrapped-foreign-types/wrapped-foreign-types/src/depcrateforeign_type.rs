// Generated macro for foreign_type (macro)
macro_rules! Depcrateforeign_type {
() => {
// Module: crate
// Provides: {"foreign_type"}
// Dependencies: {}
# [doc = " A macro to easily define wrappers for foreign types."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use foreign_types::foreign_type;"] # [doc = ""] # [doc = " # mod openssl_sys { pub type SSL = (); pub unsafe fn SSL_free(_: *mut SSL) {} pub unsafe fn SSL_dup(x: *mut SSL) -> *mut SSL {x} }"] # [doc = " # mod foo_sys { pub type THING = (); pub unsafe fn THING_free(_: *mut THING) {} }"] # [doc = " foreign_type! {"] # [doc = "     /// Documentation for the owned type."] # [doc = "     pub unsafe type Ssl: Sync + Send {"] # [doc = "         type CType = openssl_sys::SSL;"] # [doc = "         fn drop = openssl_sys::SSL_free;"] # [doc = "         fn clone = openssl_sys::SSL_dup;"] # [doc = "     }"] # [doc = ""] # [doc = "     /// This type immutably borrows other data and has a limited lifetime!"] # [doc = "     pub unsafe type Thing<'a>: Send {"] # [doc = "         type CType = foo_sys::THING;"] # [doc = "         type PhantomData = &'a ();"] # [doc = "         fn drop = foo_sys::THING_free;"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " # fn main() {}"] # [doc = " ```"] # [macro_export (local_inner_macros)] macro_rules ! foreign_type { ($ ($ t : tt) *) => { $ crate :: foreign_type_impl ! ($ crate $ ($ t) *) ; } ; }
};
}

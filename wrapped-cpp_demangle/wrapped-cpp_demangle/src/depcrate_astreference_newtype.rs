// Generated macro for reference_newtype (macro)
macro_rules! Depcrate_astreference_newtype {
() => {
// Module: crate::ast
// Provides: {"reference_newtype"}
// Dependencies: {}
macro_rules ! reference_newtype { ($ newtype_name : ident , $ oldtype : ty) => { # [derive (Debug)] struct $ newtype_name ($ oldtype) ; impl $ newtype_name { # [allow (clippy :: ptr_arg)] # [allow (unsafe_code)] fn new (types : &$ oldtype) -> &$ newtype_name { unsafe { &* (types as * const $ oldtype as * const $ newtype_name) } } } impl Drop for $ newtype_name { fn drop (& mut self) { unreachable ! ("Dropping implies we dereferenced and took ownership, which \
                              is not safe for this newtype") ; } } impl ops :: Deref for $ newtype_name { type Target = $ oldtype ; fn deref (& self) -> & Self :: Target { & self . 0 } } } ; }
};
}

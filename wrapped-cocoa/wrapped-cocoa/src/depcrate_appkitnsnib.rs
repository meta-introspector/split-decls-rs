// Generated macro for NSNib (trait)
macro_rules! Depcrate_appkitNSNib {
() => {
// Module: crate::appkit
// Provides: {"NSNib"}
// Dependencies: {}
pub trait NSNib : Sized { unsafe fn alloc (_ : Self) -> id { msg_send ! [class ! (NSNib) , alloc] } unsafe fn initWithNibNamed_bundle_ (self , name : id , bundle : id) -> id ; }
};
}

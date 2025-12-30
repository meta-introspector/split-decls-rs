// Generated macro for NSArray (trait)
macro_rules! Depcrate_foundationNSArray {
() => {
// Module: crate::foundation
// Provides: {"NSArray"}
// Dependencies: {}
pub trait NSArray : Sized { unsafe fn array (_ : Self) -> id { msg_send ! [class ! (NSArray) , array] } unsafe fn arrayWithObjects (_ : Self , objects : & [id]) -> id { msg_send ! [class ! (NSArray) , arrayWithObjects : objects . as_ptr () count : objects . len ()] } unsafe fn arrayWithObject (_ : Self , object : id) -> id { msg_send ! [class ! (NSArray) , arrayWithObject : object] } unsafe fn init (self) -> id ; unsafe fn count (self) -> NSUInteger ; unsafe fn arrayByAddingObjectFromArray (self , object : id) -> id ; unsafe fn arrayByAddingObjectsFromArray (self , objects : id) -> id ; unsafe fn objectAtIndex (self , index : NSUInteger) -> id ; }
};
}

// Generated macro for Fissile (trait)
macro_rules! Depcrate_split_producerFissile {
() => {
// Module: crate::split_producer
// Provides: {"Fissile"}
// Dependencies: {}
# [doc = " Helper trait so `&str`, `&[T]`, and `&mut [T]` can share `SplitProducer`."] pub (super) trait Fissile < P > : Sized { fn length (& self) -> usize ; fn midpoint (& self , end : usize) -> usize ; fn find (& self , separator : & P , start : usize , end : usize) -> Option < usize > ; fn rfind (& self , separator : & P , end : usize) -> Option < usize > ; fn split_once < const INCL : bool > (self , index : usize) -> (Self , Self) ; fn fold_splits < F , const INCL : bool > (self , separator : & P , folder : F , skip_last : bool) -> F where F : Folder < Self > , Self : Send ; }
};
}

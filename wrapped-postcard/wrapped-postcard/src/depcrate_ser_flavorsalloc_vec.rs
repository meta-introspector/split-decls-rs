// Generated macro for alloc_vec (module)
macro_rules! Depcrate_ser_flavorsalloc_vec {
() => {
// Module: crate::ser::flavors
// Provides: {"alloc_vec"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod alloc_vec { extern crate alloc ; use super :: Flavor ; use super :: Index ; use super :: IndexMut ; use crate :: Result ; use alloc :: vec :: Vec ; # [doc = " The `AllocVec` flavor is a wrapper type around an [`alloc::vec::Vec`]."] # [doc = ""] # [doc = " This type is only available when the (non-default) `alloc` feature is active"] # [derive (Default)] pub struct AllocVec { # [doc = " The vec to be used for serialization"] vec : Vec < u8 > , } impl AllocVec { # [doc = " Create a new, currently empty, [`alloc::vec::Vec`] to be used for storing serialized"] # [doc = " output data."] pub fn new () -> Self { Self :: default () } } impl Flavor for AllocVec { type Output = Vec < u8 > ; # [inline (always)] fn try_extend (& mut self , data : & [u8]) -> Result < () > { self . vec . extend_from_slice (data) ; Ok (()) } # [inline (always)] fn try_push (& mut self , data : u8) -> Result < () > { self . vec . push (data) ; Ok (()) } fn finalize (self) -> Result < Self :: Output > { Ok (self . vec) } } impl Index < usize > for AllocVec { type Output = u8 ; # [inline] fn index (& self , idx : usize) -> & u8 { & self . vec [idx] } } impl IndexMut < usize > for AllocVec { # [inline] fn index_mut (& mut self , idx : usize) -> & mut u8 { & mut self . vec [idx] } } }
};
}

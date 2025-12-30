// Generated macro for heapless_vec (module)
macro_rules! Depcrate_ser_flavorsheapless_vec {
() => {
// Module: crate::ser::flavors
// Provides: {"heapless_vec"}
// Dependencies: {}
# [cfg (feature = "heapless")] mod heapless_vec { use super :: Flavor ; use super :: Index ; use super :: IndexMut ; use crate :: { Error , Result } ; use heapless :: Vec ; # [doc = " The `HVec` flavor is a wrapper type around a `heapless::Vec`. This is a stack"] # [doc = " allocated data structure, with a fixed maximum size and variable amount of contents."] # [derive (Default)] pub struct HVec < const B : usize > { # [doc = " the contained data buffer"] vec : Vec < u8 , B > , } impl < const B : usize > HVec < B > { # [doc = " Create a new, currently empty, [`heapless::Vec`] to be used for storing serialized"] # [doc = " output data."] pub fn new () -> Self { Self :: default () } } impl < const B : usize > Flavor for HVec < B > { type Output = Vec < u8 , B > ; # [inline (always)] fn try_extend (& mut self , data : & [u8]) -> Result < () > { self . vec . extend_from_slice (data) . map_err (| _ | Error :: SerializeBufferFull) } # [inline (always)] fn try_push (& mut self , data : u8) -> Result < () > { self . vec . push (data) . map_err (| _ | Error :: SerializeBufferFull) } fn finalize (self) -> Result < Vec < u8 , B > > { Ok (self . vec) } } impl < const B : usize > Index < usize > for HVec < B > { type Output = u8 ; fn index (& self , idx : usize) -> & u8 { & self . vec [idx] } } impl < const B : usize > IndexMut < usize > for HVec < B > { fn index_mut (& mut self , idx : usize) -> & mut u8 { & mut self . vec [idx] } } }
};
}

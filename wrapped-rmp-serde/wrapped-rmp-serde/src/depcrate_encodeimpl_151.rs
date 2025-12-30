// Generated macro for impl_151 (impl)
macro_rules! Depcrate_encodeimpl_151 {
() => {
// Module: crate::encode
// Provides: {"impl_151"}
// Dependencies: {}
impl < 'a , W : Write + 'a , C : SerializerConfig > Serializer < W , C > { # [inline] fn maybe_unknown_len_compound < F > (& 'a mut self , len : Option < u32 > , f : F) -> Result < MaybeUnknownLengthCompound < 'a , W , C > , Error > where F : Fn (& mut W , u32) -> Result < Marker , ValueWriteError > { Ok (MaybeUnknownLengthCompound { compound : match len { Some (len) => { f (& mut self . wr , len) ? ; None } None => Some (UnknownLengthCompound :: from (& * self)) , } , se : self , }) } }
};
}

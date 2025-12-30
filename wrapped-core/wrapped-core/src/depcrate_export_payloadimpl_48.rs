// Generated macro for impl_48 (impl)
macro_rules! Depcrate_export_payloadimpl_48 {
() => {
// Module: crate::export::payload
// Provides: {"impl_48"}
// Dependencies: {}
impl DataPayload < ExportMarker > { # [doc = " Calculates a payload hash and the postcard size"] pub fn hash_and_postcard_size < H : core :: hash :: Hasher > (& self , state : & mut H) -> usize { use postcard :: ser_flavors :: Flavor ; struct HashFlavor < 'a , H > (& 'a mut H , usize) ; impl < H : core :: hash :: Hasher > Flavor for HashFlavor < '_ , H > { type Output = usize ; fn try_push (& mut self , data : u8) -> postcard :: Result < () > { self . 0 . write_u8 (data) ; self . 1 += 1 ; Ok (()) } fn finalize (self) -> postcard :: Result < Self :: Output > { Ok (self . 1) } } let mut serializer = postcard :: Serializer { output : HashFlavor (state , 0) , } ; let _infallible = self . get () . payload . serialize_yoke (& mut < dyn erased_serde :: Serializer > :: erase (& mut serializer)) ; serializer . output . 1 } }
};
}

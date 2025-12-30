// Generated macro for serialize_with_flavor (function)
macro_rules! Depcrate_serserialize_with_flavor {
() => {
// Module: crate::ser
// Provides: {"serialize_with_flavor"}
// Dependencies: {}
# [doc = " `serialize_with_flavor()` has three generic parameters, `T, F, O`."] # [doc = ""] # [doc = " * `T`: This is the type that is being serialized"] # [doc = " * `S`: This is the Storage that is used during serialization"] # [doc = " * `O`: This is the resulting storage type that is returned containing the serialized data"] # [doc = ""] # [doc = " For more information about how Flavors work, please see the"] # [doc = " [`flavors` module documentation](./flavors/index.html)."] # [doc = ""] # [doc = " ```rust"] # [doc = " use postcard2::{"] # [doc = "     serialize_with_flavor,"] # [doc = "     ser_flavors::{Cobs, Slice},"] # [doc = " };"] # [doc = ""] # [doc = " let mut buf = [0u8; 32];"] # [doc = ""] # [doc = " let data: &[u8] = &[0x01, 0x00, 0x20, 0x30];"] # [doc = " let buffer = &mut [0u8; 32];"] # [doc = " let res = serialize_with_flavor::<[u8], Cobs<Slice>, &mut [u8]>("] # [doc = "     data,"] # [doc = "     Cobs::try_new(Slice::new(buffer)).unwrap(),"] # [doc = " ).unwrap();"] # [doc = ""] # [doc = " assert_eq!(res, &[0x03, 0x04, 0x01, 0x03, 0x20, 0x30, 0x00]);"] # [doc = " ```"] pub fn serialize_with_flavor < T , S , O > (value : & T , storage : S) -> Result < O > where T : Serialize + ? Sized , S : Flavor < Output = O > , { let mut serializer = Serializer { output : storage } ; value . serialize (& mut serializer) ? ; serializer . output . finalize () . map_err (| _ | Error :: SerializeBufferFull) }
};
}

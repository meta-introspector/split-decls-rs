// Generated macro for impl_2705 (impl)
macro_rules! Depcrate_uuidimpl_2705 {
() => {
// Module: crate::uuid
// Provides: {"impl_2705"}
// Dependencies: {}
impl NSUUID { # [doc = " The 'nil UUID'."] pub fn nil () -> Retained < Self > { Self :: from_bytes ([0 ; 16]) } # [doc = " Create a new `NSUUID` from the given bytes."] # [doc = ""] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " Create a new `NSUUID` from the `uuid` crate."] # [doc = ""] # [doc = " ```ignore"] # [doc = " use uuid::Uuid;"] # [doc = " use objc2_foundation::NSUUID;"] # [doc = ""] # [doc = " let uuid: Uuid;"] # [doc = " # uuid = todo!();"] # [doc = " let obj = NSUUID::from_bytes(uuid.into_bytes());"] # [doc = " assert_eq!(obj.as_bytes(), uuid.into_bytes());"] # [doc = " ```"] pub fn from_bytes (bytes : [u8 ; 16]) -> Retained < Self > { Self :: initWithUUIDBytes (Self :: alloc () , & bytes) } # [cfg (feature = "NSString")] pub fn from_string (string : & crate :: NSString) -> Option < Retained < Self > > { Self :: initWithUUIDString (Self :: alloc () , string) } # [doc = " Convert the UUID to an array."] pub fn as_bytes (& self) -> [u8 ; 16] { let mut bytes = [0 ; 16] ; self . getUUIDBytes (& mut bytes) ; bytes } }
};
}

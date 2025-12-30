// Generated macro for ObjectMapEntry (struct)
macro_rules! Depcrate_readObjectMapEntry {
() => {
// Module: crate::read
// Provides: {"ObjectMapEntry"}
// Dependencies: {}
# [doc = " A symbol in an [`ObjectMap`]."] # [derive (Debug , Default , Clone , Copy , PartialEq , Eq , Hash)] pub struct ObjectMapEntry < 'data > { address : u64 , size : u64 , name : & 'data [u8] , object : usize , }
};
}

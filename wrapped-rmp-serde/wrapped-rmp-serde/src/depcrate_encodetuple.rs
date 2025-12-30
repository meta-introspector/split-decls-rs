// Generated macro for Tuple (struct)
macro_rules! Depcrate_encodeTuple {
() => {
// Module: crate::encode
// Provides: {"Tuple"}
// Dependencies: {}
# [doc = " Hack to store fixed-size arrays (which serde says are tuples)"] # [derive (Debug)] # [doc (hidden)] pub struct Tuple < 'a , W , C > { len : u32 , buf : Option < Vec < u8 > > , se : & 'a mut Serializer < W , C > , }
};
}

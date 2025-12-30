// Generated macro for Segment (struct)
macro_rules! Depcrate_x86_64Segment {
() => {
// Module: crate::x86_64
// Provides: {"Segment"}
// Dependencies: {}
# [repr (C)] # [derive (Copy , Debug)] pub struct Segment { pub base : u64 , pub limit : u32 , pub selector : u16 , pub _type : u8 , pub present : u8 , pub dpl : u8 , pub db : u8 , pub s : u8 , pub l : u8 , pub g : u8 , pub avl : u8 , pub unusable : u8 , pub padding : u8 , }
};
}

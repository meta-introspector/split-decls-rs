// Generated macro for IODataQueueEntry (struct)
macro_rules! Depcrate_generatedIODataQueueEntry {
() => {
// Module: crate::generated
// Provides: {"IODataQueueEntry"}
// Dependencies: {}
# [doc = " Represents an entry within the data queue"] # [doc = ""] # [doc = " This is a variable sized struct.  The data field simply represents the start of the data region.  The size of the data region is stored in the size field.  The whole size of the specific entry is the size of a UInt32 plus the size of the data region."] # [doc = " Field: size The size of the following data region."] # [doc = " Field: data Represents the beginning of the data region.  The address of the data field is a pointer to the start of the data region."] # [doc = ""] # [doc = " See also [Apple's documentation](https://developer.apple.com/documentation/iokit/iodataqueueentry?language=objc)"] # [repr (C)] # [derive (Clone , Copy , Debug , PartialEq , Default)] pub struct IODataQueueEntry { pub size : u32 , pub data : [u8 ; 4] , }
};
}

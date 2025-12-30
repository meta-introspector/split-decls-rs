// Generated macro for OSClassDescription (struct)
macro_rules! Depcrate_generatedOSClassDescription {
() => {
// Module: crate::generated
// Provides: {"OSClassDescription"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/iokit/osclassdescription?language=objc)"] # [repr (C)] # [derive (Clone , Copy , Debug , PartialEq)] pub struct OSClassDescription { pub descriptionSize : u32 , pub name : [c_char ; 96] , pub superName : [c_char ; 96] , pub methodOptionsSize : u32 , pub methodOptionsOffset : u32 , pub metaMethodOptionsSize : u32 , pub metaMethodOptionsOffset : u32 , pub queueNamesSize : u32 , pub queueNamesOffset : u32 , pub methodNamesSize : u32 , pub methodNamesOffset : u32 , pub metaMethodNamesSize : u32 , pub metaMethodNamesOffset : u32 , pub flags : u64 , pub resv1 : [u64 ; 8] , pub methodOptions : [u64 ; 0] , pub metaMethodOptions : [u64 ; 0] , pub dispatchNames : [c_char ; 0] , pub methodNames : [c_char ; 0] , pub metaMethodNames : [c_char ; 0] , }
};
}

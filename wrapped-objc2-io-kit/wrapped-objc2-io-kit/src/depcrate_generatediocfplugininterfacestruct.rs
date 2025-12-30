// Generated macro for IOCFPlugInInterfaceStruct (struct)
macro_rules! Depcrate_generatedIOCFPlugInInterfaceStruct {
() => {
// Module: crate::generated
// Provides: {"IOCFPlugInInterfaceStruct"}
// Dependencies: {}
# [doc = " [Apple's documentation](https://developer.apple.com/documentation/iokit/iocfplugininterfacestruct?language=objc)"] # [cfg (feature = "libc")] # [repr (C)] # [allow (unpredictable_function_pointer_comparisons)] # [derive (Clone , Copy , Debug , PartialEq)] pub struct IOCFPlugInInterfaceStruct { pub (crate) _reserved : * mut c_void , pub QueryInterface : Option < unsafe extern "C-unwind" fn (* mut c_void , REFIID , * mut LPVOID) -> HRESULT > , pub AddRef : Option < unsafe extern "C-unwind" fn (* mut c_void) -> ULONG > , pub Release : Option < unsafe extern "C-unwind" fn (* mut c_void) -> ULONG > , pub version : u16 , pub revision : u16 , pub Probe : Option < unsafe extern "C-unwind" fn (* mut c_void , * const CFDictionary , io_service_t , * mut i32 ,) -> IOReturn , > , pub Start : Option < unsafe extern "C-unwind" fn (* mut c_void , * const CFDictionary , io_service_t) -> IOReturn , > , pub Stop : Option < unsafe extern "C-unwind" fn (* mut c_void) -> IOReturn > , }
};
}

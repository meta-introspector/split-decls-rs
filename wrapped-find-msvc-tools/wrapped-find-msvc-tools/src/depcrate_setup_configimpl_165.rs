// Generated macro for impl_165 (impl)
macro_rules! Depcrate_setup_configimpl_165 {
() => {
// Module: crate::setup_config
// Provides: {"impl_165"}
// Dependencies: {}
impl SetupConfiguration { pub fn new () -> Result < SetupConfiguration , i32 > { let mut obj = null_mut () ; let err = unsafe { CoCreateInstance (& CLSID_SetupConfiguration , null_mut () , CLSCTX_ALL , & ISetupConfiguration :: uuidof () , & mut obj ,) } ; if err < 0 { return Err (err) ; } let obj = unsafe { ComPtr :: from_raw (obj as * mut ISetupConfiguration) } ; Ok (SetupConfiguration (obj)) } pub fn get_instance_for_current_process (& self) -> Result < SetupInstance , i32 > { let mut obj = null_mut () ; let err = unsafe { self . 0 . GetInstanceForCurrentProcess (& mut obj) } ; if err < 0 { return Err (err) ; } Ok (unsafe { SetupInstance :: from_raw (obj) }) } pub fn enum_instances (& self) -> Result < EnumSetupInstances , i32 > { let mut obj = null_mut () ; let err = unsafe { self . 0 . EnumInstances (& mut obj) } ; if err < 0 { return Err (err) ; } Ok (unsafe { EnumSetupInstances :: from_raw (obj) }) } pub fn enum_all_instances (& self) -> Result < EnumSetupInstances , i32 > { let mut obj = null_mut () ; let this = self . 0 . cast :: < ISetupConfiguration2 > () ? ; let err = unsafe { this . EnumAllInstances (& mut obj) } ; if err < 0 { return Err (err) ; } Ok (unsafe { EnumSetupInstances :: from_raw (obj) }) } }
};
}

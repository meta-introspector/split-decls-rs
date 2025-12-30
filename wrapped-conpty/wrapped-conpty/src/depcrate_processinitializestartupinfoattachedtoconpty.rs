// Generated macro for initializeStartupInfoAttachedToConPTY (function)
macro_rules! Depcrate_processinitializeStartupInfoAttachedToConPTY {
() => {
// Module: crate::process
// Provides: {"initializeStartupInfoAttachedToConPTY"}
// Dependencies: {}
fn initializeStartupInfoAttachedToConPTY (hPC : & mut HPCON) -> win :: Result < STARTUPINFOEXW > { let mut siEx = STARTUPINFOEXW :: default () ; siEx . StartupInfo . cb = size_of :: < STARTUPINFOEXW > () as u32 ; siEx . StartupInfo . hStdInput . 0 = 0 ; siEx . StartupInfo . hStdOutput . 0 = 0 ; siEx . StartupInfo . hStdError . 0 = 0 ; siEx . StartupInfo . dwFlags |= STARTF_USESTDHANDLES ; let mut size : usize = 0 ; let res = unsafe { InitializeProcThreadAttributeList (LPPROC_THREAD_ATTRIBUTE_LIST (null_mut ()) , 1 , 0 , & mut size) } ; if res . is_ok () || size == 0 { return Err (win :: Error :: new (HRESULT :: default () , "failed initialize proc attribute list" ,)) ; } let lpAttributeList = vec ! [0u8 ; size] . into_boxed_slice () ; let lpAttributeList = Box :: leak (lpAttributeList) ; siEx . lpAttributeList = LPPROC_THREAD_ATTRIBUTE_LIST (lpAttributeList . as_mut_ptr () as _) ; unsafe { InitializeProcThreadAttributeList (siEx . lpAttributeList , 1 , 0 , & mut size) ? ; UpdateProcThreadAttribute (siEx . lpAttributeList , 0 , PROC_THREAD_ATTRIBUTE_PSEUDOCONSOLE , Some (hPC . 0 as _) , size_of :: < HPCON > () , None , None ,) ? ; } Ok (siEx) }
};
}

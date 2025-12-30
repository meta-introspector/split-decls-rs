// Generated macro for macos_version (function)
macro_rules! Depcrate_fontmacos_version {
() => {
// Module: crate::font
// Provides: {"macos_version"}
// Dependencies: {}
# [cfg (test)] fn macos_version () -> (i32 , i32 , i32) { use std :: io :: Read ; let file = "/System/Library/CoreServices/SystemVersion.plist" ; let mut f = std :: fs :: File :: open (file) . unwrap () ; let mut system_version_data = Vec :: new () ; f . read_to_end (& mut system_version_data) . unwrap () ; use core_foundation :: propertylist ; let (list , _) = propertylist :: create_with_data (core_foundation :: data :: CFData :: from_buffer (& system_version_data) , propertylist :: kCFPropertyListImmutable ,) . unwrap () ; let k = unsafe { propertylist :: CFPropertyList :: wrap_under_create_rule (list) } ; let dict = unsafe { std :: mem :: transmute :: < CFDictionary , CFDictionary < CFType , CFType > > (k . downcast :: < CFDictionary > () . unwrap () ,) } ; let version = dict . find (CFString :: new ("ProductVersion") . as_CFType ()) . as_ref () . unwrap () . downcast :: < CFString > () . unwrap () . to_string () ; match version . split ('.') . map (| x | x . parse () . unwrap ()) . collect :: < Vec < _ > > () [..] { [a , b , c] => (a , b , c) , [a , b] => (a , b , 0) , _ => panic ! () , } }
};
}

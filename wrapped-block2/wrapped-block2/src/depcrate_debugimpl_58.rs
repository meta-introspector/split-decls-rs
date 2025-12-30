// Generated macro for impl_58 (impl)
macro_rules! Depcrate_debugimpl_58 {
() => {
// Module: crate::debug
// Provides: {"impl_58"}
// Dependencies: {}
impl Debug for BlockDescriptorHelper { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { if unsafe { self . descriptor . basic } . is_null () { return f . write_str ("(null)") ; } let mut f = f . debug_struct ("BlockDescriptor") ; let header = unsafe { self . descriptor . basic . as_ref () . unwrap () } ; f . field ("reserved" , & header . reserved) ; f . field ("size" , & header . size) ; match (self . has_copy_dispose , self . has_signature) { (false , false) => { } (true , false) => { let descriptor = unsafe { self . descriptor . with_copy_dispose . as_ref () . unwrap () } ; f . field ("copy" , & descriptor . copy) ; f . field ("dispose" , & descriptor . dispose) ; } (false , true) => { let descriptor = unsafe { self . descriptor . with_signature . as_ref () . unwrap () } ; f . field ("encoding" , & if descriptor . encoding . is_null () { None } else { Some (unsafe { CStr :: from_ptr (descriptor . encoding) }) } ,) ; } (true , true) => { let descriptor = unsafe { self . descriptor . with_copy_dispose_signature . as_ref () . unwrap () } ; f . field ("copy" , & descriptor . copy) ; f . field ("dispose" , & descriptor . dispose) ; f . field ("encoding" , & if descriptor . encoding . is_null () { None } else { Some (unsafe { CStr :: from_ptr (descriptor . encoding) }) } ,) ; } } f . finish () } }
};
}

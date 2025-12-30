// Generated macro for impl_191 (impl)
macro_rules! Depcrate_filedescriptorimpl_191 {
() => {
// Module: crate::filedescriptor
// Provides: {"impl_191"}
// Dependencies: {}
impl CFFileDescriptor { pub fn new (fd : RawFd , closeOnInvalidate : bool , callout : CFFileDescriptorCallBack , context : Option < & CFFileDescriptorContext > ,) -> Option < CFFileDescriptor > { let context = context . map_or (ptr :: null () , | c | c as * const _) ; unsafe { let fd_ref = CFFileDescriptorCreate (kCFAllocatorDefault , fd , closeOnInvalidate as Boolean , callout , context ,) ; if fd_ref . is_null () { None } else { Some (TCFType :: wrap_under_create_rule (fd_ref)) } } } pub fn context (& self) -> CFFileDescriptorContext { unsafe { let mut context = MaybeUninit :: < CFFileDescriptorContext > :: uninit () ; CFFileDescriptorGetContext (self . 0 , context . as_mut_ptr ()) ; context . assume_init () } } pub fn enable_callbacks (& self , callback_types : CFOptionFlags) { unsafe { CFFileDescriptorEnableCallBacks (self . 0 , callback_types) } } pub fn disable_callbacks (& self , callback_types : CFOptionFlags) { unsafe { CFFileDescriptorDisableCallBacks (self . 0 , callback_types) } } pub fn valid (& self) -> bool { unsafe { CFFileDescriptorIsValid (self . 0) != 0 } } pub fn invalidate (& self) { unsafe { CFFileDescriptorInvalidate (self . 0) } } pub fn to_run_loop_source (& self , order : CFIndex) -> Option < CFRunLoopSource > { unsafe { let source_ref = CFFileDescriptorCreateRunLoopSource (kCFAllocatorDefault , self . 0 , order) ; if source_ref . is_null () { None } else { Some (TCFType :: wrap_under_create_rule (source_ref)) } } } }
};
}

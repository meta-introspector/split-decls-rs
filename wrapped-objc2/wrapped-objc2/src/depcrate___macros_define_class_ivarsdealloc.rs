// Generated macro for dealloc (function)
macro_rules! Depcrate___macros_define_class_ivarsdealloc {
() => {
// Module: crate::__macros::define_class::ivars
// Provides: {"dealloc"}
// Dependencies: {}
# [doc = " The `dealloc` Objective-C method."] # [doc = ""] # [doc = " See the following links for more details about `dealloc`:"] # [doc = " - <https://clang.llvm.org/docs/AutomaticReferenceCounting.html#dealloc>"] # [doc = " - <https://developer.apple.com/documentation/objectivec/nsobject/1571947-dealloc>"] # [doc = " - <https://developer.apple.com/library/archive/documentation/Cocoa/Conceptual/MemoryMgmt/Articles/mmRules.html#//apple_ref/doc/uid/20000994-SW2>"] unsafe extern "C-unwind" fn dealloc < T : DefinedClass > (this : NonNull < T > , cmd : Sel) where T :: Super : ClassType , { # [doc = " Helper function for marking the cold path when branching."] # [inline] # [cold] fn cold_path () { } let drop_flag = unsafe { * ptr_to_drop_flag (this) } ; if mem :: needs_drop :: < T > () { match drop_flag { DropFlag :: Allocated | DropFlag :: InitializedIvars => cold_path () , DropFlag :: Finalized => unsafe { ptr :: drop_in_place (this . as_ptr ()) } , } } if mem :: needs_drop :: < T :: Ivars > () { match drop_flag { DropFlag :: Allocated => cold_path () , DropFlag :: InitializedIvars | DropFlag :: Finalized => { unsafe { ptr :: drop_in_place (ptr_to_ivar (this) . as_ptr ()) } ; } } } unsafe { MessageReceiver :: send_super_message (this , < T as ClassType > :: Super :: class () , cmd , () ,) } }
};
}

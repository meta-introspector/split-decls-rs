// Generated macro for msg_send_primitive (module)
macro_rules! Depcrate_runtime_message_receivermsg_send_primitive {
() => {
// Module: crate::runtime::message_receiver
// Provides: {"msg_send_primitive"}
// Dependencies: {}
# [cfg (all (not (target_vendor = "apple") , not (feature = "gnustep-1-7")))] mod msg_send_primitive { use crate :: encode :: { EncodeArguments , EncodeReturn } ; use crate :: runtime :: { AnyClass , AnyObject , Sel } ; # [track_caller] pub (crate) unsafe fn send < A : EncodeArguments , R : EncodeReturn > (_receiver : * mut AnyObject , _sel : Sel , _args : A ,) -> R { unimplemented ! ("no runtime chosen") } # [track_caller] pub (crate) unsafe fn send_super < A : EncodeArguments , R : EncodeReturn > (_receiver : * mut AnyObject , _superclass : & AnyClass , _sel : Sel , _args : A ,) -> R { unimplemented ! ("no runtime chosen") } }
};
}

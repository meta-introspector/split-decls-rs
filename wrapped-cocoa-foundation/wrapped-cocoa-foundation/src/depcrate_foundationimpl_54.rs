// Generated macro for impl_54 (impl)
macro_rules! Depcrate_foundationimpl_54 {
() => {
// Module: crate::foundation
// Provides: {"impl_54"}
// Dependencies: {}
impl NSRunLoop for id { unsafe fn currentRunLoop () -> id { msg_send ! [class ! (NSRunLoop) , currentRunLoop] } unsafe fn performSelector_target_argument_order_modes_ (self , aSelector : SEL , target : id , anArgument : id , order : NSUInteger , modes : id ,) { msg_send ! [self , performSelector : aSelector target : target argument : anArgument order : order modes : modes] } }
};
}

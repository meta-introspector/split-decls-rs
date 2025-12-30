// Generated macro for NSRunLoop (trait)
macro_rules! Depcrate_foundationNSRunLoop {
() => {
// Module: crate::foundation
// Provides: {"NSRunLoop"}
// Dependencies: {}
pub trait NSRunLoop : Sized { unsafe fn currentRunLoop () -> Self ; unsafe fn performSelector_target_argument_order_modes_ (self , aSelector : SEL , target : id , anArgument : id , order : NSUInteger , modes : id ,) ; }
};
}

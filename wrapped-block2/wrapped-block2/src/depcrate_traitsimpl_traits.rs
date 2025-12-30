// Generated macro for impl_traits (macro)
macro_rules! Depcrate_traitsimpl_traits {
() => {
// Module: crate::traits
// Provides: {"impl_traits"}
// Dependencies: {}
macro_rules ! impl_traits { ($ ($ a : ident : $ t : ident) ,*) => (impl <$ ($ t : EncodeArgument ,) * R : EncodeReturn , Closure > private :: Sealed < ($ ($ t ,) *) , R > for Closure where Closure : ? Sized + Fn ($ ($ t) ,*) -> R , { } unsafe impl <$ ($ t : EncodeArgument ,) * R : EncodeReturn > BlockFn for dyn Fn ($ ($ t) ,*) -> R + '_ { type Args = ($ ($ t ,) *) ; type Output = R ; # [inline] unsafe fn __call_block (invoke : unsafe extern "C-unwind" fn () , block : * mut Block < Self >, ($ ($ a ,) *) : Self :: Args ,) -> Self :: Output { let invoke : unsafe extern "C-unwind" fn (* mut Block < Self > $ (, $ t) *) -> R = unsafe { mem :: transmute (invoke) } ; unsafe { invoke (block $ (, $ a) *) } } } unsafe impl <'f , $ ($ t ,) * R , Closure > IntoBlock <'f , ($ ($ t ,) *) , R > for Closure where $ ($ t : EncodeArgument ,) * R : EncodeReturn , Closure : Fn ($ ($ t) ,*) -> R + 'f , { type Dyn = dyn Fn ($ ($ t) ,*) -> R + 'f ; # [inline] fn __get_invoke_stack_block () -> unsafe extern "C-unwind" fn () { unsafe extern "C-unwind" fn invoke <'f , $ ($ t ,) * R , Closure > (block : * mut StackBlock <'f , ($ ($ t ,) *) , R , Closure >, $ ($ a : $ t ,) *) -> R where Closure : Fn ($ ($ t) ,*) -> R + 'f { let closure = unsafe { &* ptr :: addr_of ! ((* block) . closure) } ; (closure) ($ ($ a) ,*) } unsafe { mem :: transmute ::< unsafe extern "C-unwind" fn (* mut StackBlock <'f , ($ ($ t ,) *) , R , Closure >, $ ($ t ,) *) -> R , unsafe extern "C-unwind" fn () , > (invoke) } } }) ; }
};
}

// Generated macro for method_impl_inner (macro)
macro_rules! Depcrate_runtime_method_implementationmethod_impl_inner {
() => {
// Module: crate::runtime::method_implementation
// Provides: {"method_impl_inner"}
// Dependencies: {}
macro_rules ! method_impl_inner { ($ (($ unsafe : ident)) ? $ abi : literal ; $ ($ t : ident) ,*) => { impl < T , R , $ ($ t) ,*> private :: Sealed for $ ($ unsafe) ? extern $ abi fn (T , Sel $ (, $ t) *) -> R where T : ? Sized + MessageReceiver , R : EncodeReturn , $ ($ t : EncodeArgument ,) * { } impl < T , R , $ ($ t) ,*> MethodImplementation for $ ($ unsafe) ? extern $ abi fn (T , Sel $ (, $ t) *) -> R where T : ? Sized + MessageReceiver , R : EncodeReturn , $ ($ t : EncodeArgument ,) * { type Callee = T :: __Inner ; type Arguments = ($ ($ t ,) *) ; type Return = R ; fn __imp (self) -> Imp { unsafe { mem :: transmute (self) } } } impl < T , $ ($ t) ,*> private :: Sealed for $ ($ unsafe) ? extern $ abi fn (Allocated < T >, Sel $ (, $ t) *) -> RetainedReturnValue where T : ? Sized + Message , $ ($ t : EncodeArgument ,) * { } # [doc (hidden)] impl < T , $ ($ t) ,*> MethodImplementation for $ ($ unsafe) ? extern $ abi fn (Allocated < T >, Sel $ (, $ t) *) -> RetainedReturnValue where T : ? Sized + Message , $ ($ t : EncodeArgument ,) * { type Callee = T ; type Arguments = ($ ($ t ,) *) ; type Return = RetainedReturnValue ; fn __imp (self) -> Imp { unsafe { mem :: transmute (self) } } } } ; }
};
}

// Generated macro for MethodImplementation (trait)
macro_rules! Depcrate_runtime_method_implementationMethodImplementation {
() => {
// Module: crate::runtime::method_implementation
// Provides: {"MethodImplementation"}
// Dependencies: {}
# [doc = " Types that can be used as the implementation of an Objective-C method."] # [doc = ""] # [doc = " This is a sealed trait that is implemented for a lot of `extern \"C\"`"] # [doc = " function pointer types."] pub trait MethodImplementation : private :: Sealed + Sized { # [doc = " The callee type of the method."] type Callee : ? Sized + RefEncode ; # [doc = " The argument types of the method."] type Arguments : EncodeArguments ; # [doc = " The return type of the method."] type Return : EncodeReturn ; # [doc (hidden)] fn __imp (self) -> Imp ; }
};
}

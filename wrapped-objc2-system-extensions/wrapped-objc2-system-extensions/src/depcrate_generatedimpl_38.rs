// Generated macro for impl_38 (impl)
macro_rules! Depcrate_generatedimpl_38 {
() => {
// Module: crate::generated
// Provides: {"impl_38"}
// Dependencies: {}
impl OSSystemExtensionManager { extern_methods ! (# [unsafe (method (init))] # [unsafe (method_family = init)] pub unsafe fn init (this : Allocated < Self >) -> Retained < Self >; # [unsafe (method (new))] # [unsafe (method_family = new)] pub unsafe fn new (& self) -> Retained < Self >; # [unsafe (method (sharedManager))] # [unsafe (method_family = none)] pub unsafe fn sharedManager () -> Retained < OSSystemExtensionManager >; # [doc = " Submits a System Extension request to the manager."] # [doc = ""] # [doc = ""] # [doc = " Parameter `request`: The request to process."] # [unsafe (method (submitRequest :))] # [unsafe (method_family = none)] pub unsafe fn submitRequest (& self , request : & OSSystemExtensionRequest) ;) ; }
};
}

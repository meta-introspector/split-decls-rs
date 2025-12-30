// Generated macro for IfaceBuilder (struct)
macro_rules! Depcrate_ifacedescIfaceBuilder {
() => {
// Module: crate::ifacedesc
// Provides: {"IfaceBuilder"}
// Dependencies: {}
# [doc = " Struct used to build an interface."] # [doc = ""] # [doc = " You get an instance of this struct in the call to Crossroads::register."] # [doc = ""] # [doc = " Register new methods, properties and signals using the corresponding functions on this struct."] # [doc = " You might find several similar functions, e g `method`, `method_with_cr`, `method_with_cr_async` and"] # [doc = " `method_with_cr_custom`. Methods that have \"with_cr\" will allow you to access the full mutable Crossroads"] # [doc = " instance, but beware - trying to recursively handle methods from within a method handler is not allowed"] # [doc = " and may cause panics."] # [doc = ""] # [doc = " Methods that have \"_async\" will allow you to defer the result of your method. During await points,"] # [doc = " other tasks with method calls can run as separate tasks. Remember to call Crossroads::set_async_support"] # [doc = " when using async methods."] # [doc = ""] # [derive (Debug)] pub struct IfaceBuilder < T : Send + 'static > (IfaceDesc , PhantomData < & 'static T >) ;
};
}

// Generated macro for StaticComObject (struct)
macro_rules! Depcrate_com_objectStaticComObject {
() => {
// Module: crate::com_object
// Provides: {"StaticComObject"}
// Dependencies: {}
# [doc = " Enables applications to define COM objects using static storage. This is useful for factory"] # [doc = " objects, stateless objects, or objects which use need to contain or use mutable global state."] # [doc = ""] # [doc = " COM objects that are defined using `StaticComObject` have their storage placed directly in"] # [doc = " static storage; they are not stored in the heap."] # [doc = ""] # [doc = " COM objects defined using `StaticComObject` do have a reference count and this reference"] # [doc = " count is adjusted when owned COM interface references (e.g. `IFoo` and `IUnknown`) are created"] # [doc = " for the object. The reference count is initialized to 1."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " #[implement(IFoo)]"] # [doc = " struct MyApp {"] # [doc = "     // ..."] # [doc = " }"] # [doc = ""] # [doc = " static MY_STATIC_APP: StaticComObject<MyApp> = MyApp { ... }.into_static();"] # [doc = ""] # [doc = " fn get_my_static_ifoo() -> IFoo {"] # [doc = "     MY_STATIC_APP.to_interface()"] # [doc = " }"] # [doc = " ```"] pub struct StaticComObject < T > where T : ComObjectInner , { outer : T :: Outer , }
};
}

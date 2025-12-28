macro_rules! InterfaceRef {
    () => {
        # [doc = " This has the same memory representation as `IFoo`, but represents a borrowed interface pointer."] # [doc = ""] # [doc = " This type has no `Drop` impl; it does not AddRef/Release the given interface. However, because"] # [doc = " it has a lifetime parameter, it always represents a non-null pointer to an interface."] # [repr (transparent)] pub struct InterfaceRef < 'a , I > (NonNull < c_void > , PhantomData < & 'a I >) ;
    };
}

InterfaceRef!()
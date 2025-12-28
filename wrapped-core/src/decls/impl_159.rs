macro_rules! deps {
    () => {
        InterfaceRef!();
        Interface!();
    };
}

macro_rules! impl_159 {
    () => {
        deps!();
        impl < I : Interface > InterfaceRef < '_ , I > { # [doc = " Creates an `InterfaceRef` from a raw pointer. _This is extremely dangerous, since there"] # [doc = " is no lifetime tracking at all!_"] # [doc = ""] # [doc = " # Safety"] # [doc = " The caller must guarantee that the `'a` lifetime parameter is bound by context to a correct"] # [doc = " lifetime."] # [inline (always)] pub unsafe fn from_raw (ptr : NonNull < c_void >) -> Self { Self (ptr , PhantomData) } # [doc = " Creates an `InterfaceRef` from an interface reference. This safely associates the lifetime"] # [doc = " of the interface reference with the `'a` parameter of `InterfaceRef`. This allows for"] # [doc = " lifetime checking _without_ calling AddRef/Release on the underlying lifetime, which can"] # [doc = " improve efficiency."] # [inline (always)] pub fn from_interface (interface : & I) -> Self { unsafe { Self (NonNull :: new_unchecked (interface . as_raw ()) , PhantomData) } } # [doc = " Calls AddRef on the underlying COM interface and returns an \"owned\" (counted) reference."] # [inline (always)] pub fn to_owned (self) -> I { (* self) . clone () } }
    };
}

impl_159!()
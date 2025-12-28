macro_rules! AsImpl {
    () => {
        # [doc = " A trait for retrieving the implementation behind a COM or WinRT interface."] # [doc = ""] # [doc = " This trait is automatically implemented when using the `implement` macro."] pub trait AsImpl < T > { # [doc = " # Safety"] # [doc = ""] # [doc = " The caller needs to ensure that `self` is actually implemented by the"] # [doc = " implementation `T`."] unsafe fn as_impl (& self) -> & T { unsafe { self . as_impl_ptr () . as_ref () } } # [doc = " Returns a pointer to the implementation object."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller needs to ensure that `self` is actually implemented by the"] # [doc = " implementation `T`."] unsafe fn as_impl_ptr (& self) -> core :: ptr :: NonNull < T > ; }
    };
}

AsImpl!()
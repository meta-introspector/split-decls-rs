macro_rules! deps {
    () => {
        Unique!();
    };
}

macro_rules! impl_207 {
    () => {
        deps!();
        impl < T : ? Sized > Unique < T > { # [doc = " Creates a new `Unique`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `ptr` must be non-null."] # [inline] pub const unsafe fn new_unchecked (ptr : * mut T) -> Self { unsafe { Unique { pointer : NonNull :: new_unchecked (ptr) , _marker : PhantomData , } } } # [doc = " Acquires the underlying `*mut` pointer."] # [must_use = "`self` will be dropped if the result is not used"] # [inline] pub const fn as_ptr (self) -> * mut T { self . pointer . as_ptr () } # [doc = " Acquires the underlying `*mut` pointer."] # [must_use = "`self` will be dropped if the result is not used"] # [inline] pub const fn as_non_null_ptr (self) -> NonNull < T > { self . pointer } # [doc = " Dereferences the content."] # [doc = ""] # [doc = " The resulting lifetime is bound to self so this behaves \"as if\""] # [doc = " it were actually an instance of T that is getting borrowed. If a longer"] # [doc = " (unbound) lifetime is needed, use `&*my_ptr.as_ptr()`."] # [must_use] # [inline] pub const unsafe fn as_ref (& self) -> & T { unsafe { & * (self . as_ptr () as * const T) } } # [doc = " Mutably dereferences the content."] # [doc = ""] # [doc = " The resulting lifetime is bound to self so this behaves \"as if\""] # [doc = " it were actually an instance of T that is getting borrowed. If a longer"] # [doc = " (unbound) lifetime is needed, use `&mut *my_ptr.as_ptr()`."] # [must_use] # [inline] pub unsafe fn as_mut (& mut self) -> & mut T { unsafe { self . pointer . as_mut () } } }
    };
}

impl_207!()
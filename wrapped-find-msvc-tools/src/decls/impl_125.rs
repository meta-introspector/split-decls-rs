macro_rules! deps {
    () => {
        Interface!();
        ComPtr!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < T > ComPtr < T > where T : Interface , { # [doc = " Creates a `ComPtr` to wrap a raw pointer."] # [doc = " It takes ownership over the pointer which means it does __not__ call `AddRef`."] # [doc = " `T` __must__ be a COM interface that inherits from `IUnknown`."] pub unsafe fn from_raw (ptr : * mut T) -> ComPtr < T > { assert ! (! ptr . is_null ()) ; ComPtr (ptr) } # [doc = " For internal use only."] fn as_unknown (& self) -> & IUnknown { unsafe { & * (self . 0 as * mut IUnknown) } } # [doc = " Performs `QueryInterface` fun."] pub fn cast < U > (& self) -> Result < ComPtr < U > , i32 > where U : Interface , { let mut obj = null_mut () ; let err = unsafe { self . as_unknown () . QueryInterface (& U :: uuidof () , & mut obj) } ; if err < 0 { return Err (err) ; } Ok (unsafe { ComPtr :: from_raw (obj as * mut U) }) } }
    };
}

impl_125!();
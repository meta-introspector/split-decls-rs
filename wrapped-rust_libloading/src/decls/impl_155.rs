macro_rules! deps {
    () => {
        Library!();
        Symbol!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < 'lib , T > Symbol < 'lib , T > { # [doc = " Extract the wrapped `os::platform::Symbol`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Using this function relinquishes all the lifetime guarantees. It is up to the developer to"] # [doc = " ensure the resulting `Symbol` is not used past the lifetime of the `Library` this symbol"] # [doc = " was loaded from."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use ::libloading::{Library, Symbol};"] # [doc = " unsafe {"] # [doc = "     let lib = Library::new(\"/path/to/awesome.module\").unwrap();"] # [doc = "     let symbol: Symbol<*mut u32> = lib.get(b\"symbol\\0\").unwrap();"] # [doc = "     let symbol = symbol.into_raw();"] # [doc = " }"] # [doc = " ```"] pub unsafe fn into_raw (self) -> imp :: Symbol < T > { self . inner } # [doc = " Wrap the `os::platform::Symbol` into this safe wrapper."] # [doc = ""] # [doc = " Note that, in order to create association between the symbol and the library this symbol"] # [doc = " came from, this function requires a reference to the library."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The `library` reference must be exactly the library `sym` was loaded from."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```no_run"] # [doc = " # use ::libloading::{Library, Symbol};"] # [doc = " unsafe {"] # [doc = "     let lib = Library::new(\"/path/to/awesome.module\").unwrap();"] # [doc = "     let symbol: Symbol<*mut u32> = lib.get(b\"symbol\\0\").unwrap();"] # [doc = "     let symbol = symbol.into_raw();"] # [doc = "     let symbol = Symbol::from_raw(symbol, &lib);"] # [doc = " }"] # [doc = " ```"] pub unsafe fn from_raw < L > (sym : imp :: Symbol < T > , library : & 'lib L) -> Symbol < 'lib , T > { let _ = library ; Symbol { inner : sym , pd : marker :: PhantomData , } } # [doc = " Try to convert the symbol into a raw pointer."] # [doc = " Success depends on the platform. Currently, this fn always succeeds and returns some."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Using this function relinquishes all the lifetime guarantees. It is up to the developer to"] # [doc = " ensure the resulting `Symbol` is not used past the lifetime of the `Library` this symbol"] # [doc = " was loaded from."] pub unsafe fn try_as_raw_ptr (self) -> Option < * mut core :: ffi :: c_void > { Some (unsafe { self . into_raw () } . as_raw_ptr () ,) } }
    };
}

impl_155!();
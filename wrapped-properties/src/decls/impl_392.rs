macro_rules! deps {
    () => {
        ScriptWithExtensionsBorrowed!();
        Baked!();
        ScriptWithExtensions!();
    };
}

macro_rules! impl_392 {
    () => {
        deps!();
        impl ScriptWithExtensionsBorrowed < 'static > { # [doc = " Creates a new instance of `ScriptWithExtensionsBorrowed` using compiled data."] # [doc = ""] # [doc = " ✨ *Enabled with the `compiled_data` Cargo feature.*"] # [doc = ""] # [doc = " [📚 Help choosing a constructor](icu_provider::constructors)"] # [cfg (feature = "compiled_data")] pub fn new () -> Self { Self { data : crate :: provider :: Baked :: SINGLETON_PROPERTY_SCRIPT_WITH_EXTENSIONS_V1 , } } # [doc = " Cheaply converts a [`ScriptWithExtensionsBorrowed<'static>`] into a [`ScriptWithExtensions`]."] # [doc = ""] # [doc = " Note: Due to branching and indirection, using [`ScriptWithExtensions`] might inhibit some"] # [doc = " compile-time optimizations that are possible with [`ScriptWithExtensionsBorrowed`]."] pub const fn static_to_owned (self) -> ScriptWithExtensions { ScriptWithExtensions { data : DataPayload :: from_static_ref (self . data) , } } }
    };
}

impl_392!();
macro_rules! deps {
    () => {
        Interface!();
        Guid!();
    };
}

macro_rules! interface {
    () => {
        deps!();
        # [doc = " Defines a COM interface to call or implement."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust,no_run"] # [doc = " use windows_core::*;"] # [doc = ""] # [doc = " #[interface(\"094d70d6-5202-44b8-abb8-43860da5aca2\")]"] # [doc = " unsafe trait IValue: IUnknown {"] # [doc = "     fn GetValue(&self, value: *mut i32) -> HRESULT;"] # [doc = " }"] # [doc = ""] # [doc = " #[implement(IValue)]"] # [doc = " struct Value(i32);"] # [doc = ""] # [doc = " impl IValue_Impl for Value_Impl {"] # [doc = "     unsafe fn GetValue(&self, value: *mut i32) -> HRESULT {"] # [doc = "         *value = self.0;"] # [doc = "         HRESULT(0)"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let object: IValue = Value(123).into();"] # [doc = " // Call interface methods..."] # [doc = " ```"] # [proc_macro_attribute] pub fn interface (attributes : proc_macro :: TokenStream , original_type : proc_macro :: TokenStream ,) -> proc_macro :: TokenStream { let guid = syn :: parse_macro_input ! (attributes as Guid) ; let interface = syn :: parse_macro_input ! (original_type as Interface) ; let tokens = match interface . gen_tokens (& guid) { Ok (t) => t , Err (e) => return e . to_compile_error () . into () , } ; tokens . into () }
    };
}

interface!();
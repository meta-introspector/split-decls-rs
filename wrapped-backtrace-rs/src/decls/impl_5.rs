macro_rules! deps {
    () => {
        Frame!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Frame { # [doc = " Returns the current instruction pointer of this frame."] # [doc = ""] # [doc = " This is normally the next instruction to execute in the frame, but not"] # [doc = " all implementations list this with 100% accuracy (but it's generally"] # [doc = " pretty close)."] # [doc = ""] # [doc = " It is recommended to pass this value to `backtrace::resolve` to turn it"] # [doc = " into a symbol name."] pub fn ip (& self) -> * mut c_void { self . inner . ip () } # [doc = " Returns the current stack pointer of this frame."] # [doc = ""] # [doc = " In the case that a backend cannot recover the stack pointer for this"] # [doc = " frame, a null pointer is returned."] pub fn sp (& self) -> * mut c_void { self . inner . sp () } # [doc = " Returns the starting symbol address of the frame of this function."] # [doc = ""] # [doc = " This will attempt to rewind the instruction pointer returned by `ip` to"] # [doc = " the start of the function, returning that value. In some cases, however,"] # [doc = " backends will just return `ip` from this function."] # [doc = ""] # [doc = " The returned value can sometimes be used if `backtrace::resolve` failed"] # [doc = " on the `ip` given above."] pub fn symbol_address (& self) -> * mut c_void { self . inner . symbol_address () } # [doc = " Returns the base address of the module to which the frame belongs."] pub fn module_base_address (& self) -> Option < * mut c_void > { self . inner . module_base_address () } }
    };
}

impl_5!();
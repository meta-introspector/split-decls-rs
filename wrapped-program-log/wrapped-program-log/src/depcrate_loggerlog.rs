// Generated macro for Log (trait)
macro_rules! Depcrate_loggerLog {
() => {
// Module: crate::logger
// Provides: {"Log"}
// Dependencies: {}
# [doc = " Trait to specify the log behavior for a type."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The implementation must ensure that the value returned by any of the methods correctly"] # [doc = " reflects the actual number of bytes written to the buffer. Returning a value greater"] # [doc = " than the number of bytes written to the buffer will result in undefined behavior, since"] # [doc = " it will lead to reading uninitialized memory from the buffer."] pub unsafe trait Log { # [inline (always)] fn debug (& self , buffer : & mut [MaybeUninit < u8 >]) -> usize { self . debug_with_args (buffer , & []) } # [inline (always)] fn debug_with_args (& self , buffer : & mut [MaybeUninit < u8 >] , args : & [Argument]) -> usize { self . write_with_args (buffer , args) } # [inline (always)] fn write (& self , buffer : & mut [MaybeUninit < u8 >]) -> usize { self . write_with_args (buffer , & []) } fn write_with_args (& self , buffer : & mut [MaybeUninit < u8 >] , parameters : & [Argument]) -> usize ; }
};
}

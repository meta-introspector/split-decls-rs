macro_rules! deps {
    () => {
        ExternAbi!();
        CVariadicStatus!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl ExternAbi { # [doc = " An ABI \"like Rust\""] # [doc = ""] # [doc = " These ABIs are fully controlled by the Rust compiler, which means they"] # [doc = " - support unwinding with `-Cpanic=unwind`, unlike `extern \"C\"`"] # [doc = " - often diverge from the C ABI"] # [doc = " - are subject to change between compiler versions"] pub fn is_rustic_abi (self) -> bool { use ExternAbi :: * ; matches ! (self , Rust | RustCall | RustCold) } # [doc = " Returns whether the ABI supports C variadics. This only controls whether we allow *imports*"] # [doc = " of such functions via `extern` blocks; there's a separate check during AST construction"] # [doc = " guarding *definitions* of variadic functions."] # [cfg (feature = "nightly")] pub fn supports_c_variadic (self) -> CVariadicStatus { match self { Self :: C { .. } | Self :: Cdecl { .. } | Self :: Aapcs { .. } | Self :: Win64 { .. } | Self :: SysV64 { .. } | Self :: EfiApi => CVariadicStatus :: Stable , Self :: System { .. } => { CVariadicStatus :: Unstable { feature : rustc_span :: sym :: extern_system_varargs } } _ => CVariadicStatus :: NotSupported , } } }
    };
}

impl_27!();
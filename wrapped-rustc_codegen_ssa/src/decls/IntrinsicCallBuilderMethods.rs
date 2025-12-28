macro_rules! deps {
    () => {
        BackendTypes!();
        PlaceRef!();
        OperandRef!();
    };
}

macro_rules! IntrinsicCallBuilderMethods {
    () => {
        deps!();
        pub trait IntrinsicCallBuilderMethods < 'tcx > : BackendTypes { # [doc = " Higher-level interface to emitting calls to intrinsics"] # [doc = ""] # [doc = " Remember to add all intrinsics here, in `compiler/rustc_hir_analysis/src/check/mod.rs`,"] # [doc = " and in `library/core/src/intrinsics.rs`; if you need access to any LLVM intrinsics,"] # [doc = " add them to `compiler/rustc_codegen_llvm/src/context.rs`."] # [doc = " Returns `Err` if another instance should be called instead. This is used to invoke"] # [doc = " intrinsic default bodies in case an intrinsic is not implemented by the backend."] # [doc = ""] # [doc = " NOTE: allowed to call [`BuilderMethods::call`]"] # [doc = ""] # [doc = " [`BuilderMethods::call`]: super::builder::BuilderMethods::call"] fn codegen_intrinsic_call (& mut self , instance : ty :: Instance < 'tcx > , args : & [OperandRef < 'tcx , Self :: Value >] , result_dest : PlaceRef < 'tcx , Self :: Value > , span : Span ,) -> Result < () , ty :: Instance < 'tcx > > ; fn abort (& mut self) ; fn assume (& mut self , val : Self :: Value) ; fn expect (& mut self , cond : Self :: Value , expected : bool) -> Self :: Value ; # [doc = " Trait method used to load a function while testing if it is associated with a type"] # [doc = " identifier."] fn type_checked_load (& mut self , llvtable : Self :: Value , vtable_byte_offset : u64 , typeid : Self :: Metadata ,) -> Self :: Value ; # [doc = " Trait method used to inject `va_start` on the \"spoofed\" `VaListImpl` in"] # [doc = " Rust defined C-variadic functions."] fn va_start (& mut self , val : Self :: Value) -> Self :: Value ; # [doc = " Trait method used to inject `va_end` on the \"spoofed\" `VaListImpl` before"] # [doc = " Rust defined C-variadic functions return."] fn va_end (& mut self , val : Self :: Value) -> Self :: Value ; }
    };
}

IntrinsicCallBuilderMethods!();
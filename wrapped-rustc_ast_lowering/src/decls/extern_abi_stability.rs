macro_rules! deps {
    () => {
        UnstableAbi!();
        GateReason!();
    };
}

macro_rules! extern_abi_stability {
    () => {
        deps!();
        pub fn extern_abi_stability (abi : ExternAbi) -> Result < () , UnstableAbi > { match abi { ExternAbi :: Rust | ExternAbi :: C { .. } | ExternAbi :: Cdecl { .. } | ExternAbi :: Stdcall { .. } | ExternAbi :: Fastcall { .. } | ExternAbi :: Thiscall { .. } | ExternAbi :: Aapcs { .. } | ExternAbi :: Win64 { .. } | ExternAbi :: SysV64 { .. } | ExternAbi :: System { .. } | ExternAbi :: EfiApi => Ok (()) , ExternAbi :: Unadjusted => { Err (UnstableAbi { abi , feature : sym :: abi_unadjusted , explain : GateReason :: ImplDetail }) } ExternAbi :: Vectorcall { .. } => Err (UnstableAbi { abi , feature : sym :: abi_vectorcall , explain : GateReason :: Experimental , }) , ExternAbi :: RustCall => Err (UnstableAbi { abi , feature : sym :: unboxed_closures , explain : GateReason :: Experimental , }) , ExternAbi :: RustCold => { Err (UnstableAbi { abi , feature : sym :: rust_cold_cc , explain : GateReason :: Experimental }) } ExternAbi :: RustInvalid => { Err (UnstableAbi { abi , feature : sym :: rustc_attrs , explain : GateReason :: ImplDetail }) } ExternAbi :: GpuKernel => Err (UnstableAbi { abi , feature : sym :: abi_gpu_kernel , explain : GateReason :: Experimental , }) , ExternAbi :: PtxKernel => { Err (UnstableAbi { abi , feature : sym :: abi_ptx , explain : GateReason :: Experimental }) } ExternAbi :: Msp430Interrupt => Err (UnstableAbi { abi , feature : sym :: abi_msp430_interrupt , explain : GateReason :: Experimental , }) , ExternAbi :: X86Interrupt => Err (UnstableAbi { abi , feature : sym :: abi_x86_interrupt , explain : GateReason :: Experimental , }) , ExternAbi :: AvrInterrupt | ExternAbi :: AvrNonBlockingInterrupt => Err (UnstableAbi { abi , feature : sym :: abi_avr_interrupt , explain : GateReason :: Experimental , }) , ExternAbi :: RiscvInterruptM | ExternAbi :: RiscvInterruptS => Err (UnstableAbi { abi , feature : sym :: abi_riscv_interrupt , explain : GateReason :: Experimental , }) , ExternAbi :: CmseNonSecureCall => Err (UnstableAbi { abi , feature : sym :: abi_cmse_nonsecure_call , explain : GateReason :: Experimental , }) , ExternAbi :: CmseNonSecureEntry => Err (UnstableAbi { abi , feature : sym :: cmse_nonsecure_entry , explain : GateReason :: Experimental , }) , ExternAbi :: Custom => { Err (UnstableAbi { abi , feature : sym :: abi_custom , explain : GateReason :: Experimental }) } } }
    };
}

extern_abi_stability!()
macro_rules! deps {
    () => {
        ExternAbi!();
    };
}

macro_rules! macro_17 {
    () => {
        deps!();
        abi_impls ! { ExternAbi = { C { unwind : false } =><= "C" , C { unwind : true } =><= "C-unwind" , Rust =><= "Rust" , Aapcs { unwind : false } =><= "aapcs" , Aapcs { unwind : true } =><= "aapcs-unwind" , AvrInterrupt =><= "avr-interrupt" , AvrNonBlockingInterrupt =><= "avr-non-blocking-interrupt" , Cdecl { unwind : false } =><= "cdecl" , Cdecl { unwind : true } =><= "cdecl-unwind" , CmseNonSecureCall =><= "cmse-nonsecure-call" , CmseNonSecureEntry =><= "cmse-nonsecure-entry" , Custom =><= "custom" , EfiApi =><= "efiapi" , Fastcall { unwind : false } =><= "fastcall" , Fastcall { unwind : true } =><= "fastcall-unwind" , GpuKernel =><= "gpu-kernel" , Msp430Interrupt =><= "msp430-interrupt" , PtxKernel =><= "ptx-kernel" , RiscvInterruptM =><= "riscv-interrupt-m" , RiscvInterruptS =><= "riscv-interrupt-s" , RustCall =><= "rust-call" , RustCold =><= "rust-cold" , RustInvalid =><= "rust-invalid" , Stdcall { unwind : false } =><= "stdcall" , Stdcall { unwind : true } =><= "stdcall-unwind" , System { unwind : false } =><= "system" , System { unwind : true } =><= "system-unwind" , SysV64 { unwind : false } =><= "sysv64" , SysV64 { unwind : true } =><= "sysv64-unwind" , Thiscall { unwind : false } =><= "thiscall" , Thiscall { unwind : true } =><= "thiscall-unwind" , Unadjusted =><= "unadjusted" , Vectorcall { unwind : false } =><= "vectorcall" , Vectorcall { unwind : true } =><= "vectorcall-unwind" , Win64 { unwind : false } =><= "win64" , Win64 { unwind : true } =><= "win64-unwind" , X86Interrupt =><= "x86-interrupt" , } }
    };
}

macro_17!();
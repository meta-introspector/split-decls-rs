macro_rules! Architecture {
    () => {
        # [doc = " A CPU architecture."] # [allow (missing_docs)] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] # [non_exhaustive] pub enum Architecture { Unknown , Aarch64 , # [allow (non_camel_case_types)] Aarch64_Ilp32 , Alpha , Arm , Avr , Bpf , Csky , E2K32 , E2K64 , I386 , X86_64 , # [allow (non_camel_case_types)] X86_64_X32 , Hexagon , Hppa , LoongArch32 , LoongArch64 , M68k , Mips , Mips64 , # [allow (non_camel_case_types)] Mips64_N32 , Msp430 , PowerPc , PowerPc64 , Riscv32 , Riscv64 , S390x , Sbf , Sharc , Sparc , Sparc32Plus , Sparc64 , SuperH , Wasm32 , Wasm64 , Xtensa , }
    };
}

Architecture!();
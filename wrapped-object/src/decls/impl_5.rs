macro_rules! deps {
    () => {
        AddressSize!();
        U64!();
        U32!();
        Architecture!();
        U16!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl Architecture { # [doc = " The size of an address value for this architecture."] # [doc = ""] # [doc = " Returns `None` for unknown architectures."] pub fn address_size (self) -> Option < AddressSize > { match self { Architecture :: Unknown => None , Architecture :: Aarch64 => Some (AddressSize :: U64) , Architecture :: Aarch64_Ilp32 => Some (AddressSize :: U32) , Architecture :: Alpha => Some (AddressSize :: U64) , Architecture :: Arm => Some (AddressSize :: U32) , Architecture :: Avr => Some (AddressSize :: U8) , Architecture :: Bpf => Some (AddressSize :: U64) , Architecture :: Csky => Some (AddressSize :: U32) , Architecture :: E2K32 => Some (AddressSize :: U32) , Architecture :: E2K64 => Some (AddressSize :: U64) , Architecture :: I386 => Some (AddressSize :: U32) , Architecture :: X86_64 => Some (AddressSize :: U64) , Architecture :: X86_64_X32 => Some (AddressSize :: U32) , Architecture :: Hexagon => Some (AddressSize :: U32) , Architecture :: Hppa => Some (AddressSize :: U32) , Architecture :: LoongArch32 => Some (AddressSize :: U32) , Architecture :: LoongArch64 => Some (AddressSize :: U64) , Architecture :: M68k => Some (AddressSize :: U32) , Architecture :: Mips => Some (AddressSize :: U32) , Architecture :: Mips64 => Some (AddressSize :: U64) , Architecture :: Mips64_N32 => Some (AddressSize :: U32) , Architecture :: Msp430 => Some (AddressSize :: U16) , Architecture :: PowerPc => Some (AddressSize :: U32) , Architecture :: PowerPc64 => Some (AddressSize :: U64) , Architecture :: Riscv32 => Some (AddressSize :: U32) , Architecture :: Riscv64 => Some (AddressSize :: U64) , Architecture :: S390x => Some (AddressSize :: U64) , Architecture :: Sbf => Some (AddressSize :: U64) , Architecture :: Sharc => Some (AddressSize :: U32) , Architecture :: Sparc => Some (AddressSize :: U32) , Architecture :: Sparc32Plus => Some (AddressSize :: U32) , Architecture :: Sparc64 => Some (AddressSize :: U64) , Architecture :: Wasm32 => Some (AddressSize :: U32) , Architecture :: Wasm64 => Some (AddressSize :: U64) , Architecture :: Xtensa => Some (AddressSize :: U32) , Architecture :: SuperH => Some (AddressSize :: U32) , } } }
    };
}

impl_5!();
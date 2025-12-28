macro_rules! deps {
    () => {
        ReadRef!();
        Architecture!();
        Pod!();
        Result!();
    };
}

macro_rules! FatArch {
    () => {
        deps!();
        # [doc = " A trait for generic access to [`macho::FatArch32`] and [`macho::FatArch64`]."] # [allow (missing_docs)] pub trait FatArch : Pod { type Word : Into < u64 > ; const MAGIC : u32 ; fn cputype (& self) -> u32 ; fn cpusubtype (& self) -> u32 ; fn offset (& self) -> Self :: Word ; fn size (& self) -> Self :: Word ; fn align (& self) -> u32 ; fn architecture (& self) -> Architecture { match self . cputype () { macho :: CPU_TYPE_ARM => Architecture :: Arm , macho :: CPU_TYPE_ARM64 => Architecture :: Aarch64 , macho :: CPU_TYPE_X86 => Architecture :: I386 , macho :: CPU_TYPE_X86_64 => Architecture :: X86_64 , macho :: CPU_TYPE_MIPS => Architecture :: Mips , macho :: CPU_TYPE_POWERPC => Architecture :: PowerPc , macho :: CPU_TYPE_POWERPC64 => Architecture :: PowerPc64 , _ => Architecture :: Unknown , } } fn file_range (& self) -> (u64 , u64) { (self . offset () . into () , self . size () . into ()) } fn data < 'data , R : ReadRef < 'data > > (& self , file : R) -> Result < & 'data [u8] > { file . read_bytes_at (self . offset () . into () , self . size () . into ()) . read_error ("Invalid fat arch offset or size") } }
    };
}

FatArch!()
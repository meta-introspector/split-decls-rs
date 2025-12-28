macro_rules! deps {
    () => {
        U32!();
        Endian!();
    };
}

macro_rules! Section32 {
    () => {
        deps!();
        # [doc = " 32-bit section."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Section32 < E : Endian > { # [doc = " name of this section"] pub sectname : [u8 ; 16] , # [doc = " segment this section goes in"] pub segname : [u8 ; 16] , # [doc = " memory address of this section"] pub addr : U32 < E > , # [doc = " size in bytes of this section"] pub size : U32 < E > , # [doc = " file offset of this section"] pub offset : U32 < E > , # [doc = " section alignment (power of 2)"] pub align : U32 < E > , # [doc = " file offset of relocation entries"] pub reloff : U32 < E > , # [doc = " number of relocation entries"] pub nreloc : U32 < E > , # [doc = " flags (section type and attributes)"] pub flags : U32 < E > , # [doc = " reserved (for offset or index)"] pub reserved1 : U32 < E > , # [doc = " reserved (for count or sizeof)"] pub reserved2 : U32 < E > , }
    };
}

Section32!()
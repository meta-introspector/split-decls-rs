macro_rules! deps {
    () => {
        Endian!();
        U32!();
        U64!();
    };
}

macro_rules! Section64 {
    () => {
        deps!();
        # [doc = " 64-bit section."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct Section64 < E : Endian > { # [doc = " name of this section"] pub sectname : [u8 ; 16] , # [doc = " segment this section goes in"] pub segname : [u8 ; 16] , # [doc = " memory address of this section"] pub addr : U64 < E > , # [doc = " size in bytes of this section"] pub size : U64 < E > , # [doc = " file offset of this section"] pub offset : U32 < E > , # [doc = " section alignment (power of 2)"] pub align : U32 < E > , # [doc = " file offset of relocation entries"] pub reloff : U32 < E > , # [doc = " number of relocation entries"] pub nreloc : U32 < E > , # [doc = " flags (section type and attributes)"] pub flags : U32 < E > , # [doc = " reserved (for offset or index)"] pub reserved1 : U32 < E > , # [doc = " reserved (for count or sizeof)"] pub reserved2 : U32 < E > , # [doc = " reserved"] pub reserved3 : U32 < E > , }
    };
}

Section64!()
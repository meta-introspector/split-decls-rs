macro_rules! deps {
    () => {
        U32!();
        U64!();
        BigEndian!();
    };
}

macro_rules! FatArch64 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct FatArch64 { # [doc = " cpu specifier (int)"] pub cputype : U32 < BigEndian > , # [doc = " machine specifier (int)"] pub cpusubtype : U32 < BigEndian > , # [doc = " file offset to this object file"] pub offset : U64 < BigEndian > , # [doc = " size of this object file"] pub size : U64 < BigEndian > , # [doc = " alignment as a power of 2"] pub align : U32 < BigEndian > , # [doc = " reserved"] pub reserved : U32 < BigEndian > , }
    };
}

FatArch64!()
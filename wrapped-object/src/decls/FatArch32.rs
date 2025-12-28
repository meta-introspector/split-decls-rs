macro_rules! deps {
    () => {
        BigEndian!();
        U32!();
    };
}

macro_rules! FatArch32 {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct FatArch32 { # [doc = " cpu specifier (int)"] pub cputype : U32 < BigEndian > , # [doc = " machine specifier (int)"] pub cpusubtype : U32 < BigEndian > , # [doc = " file offset to this object file"] pub offset : U32 < BigEndian > , # [doc = " size of this object file"] pub size : U32 < BigEndian > , # [doc = " alignment as a power of 2"] pub align : U32 < BigEndian > , }
    };
}

FatArch32!();
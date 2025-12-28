macro_rules! deps {
    () => {
        BigEndian!();
        U32!();
        Endian!();
    };
}

macro_rules! MachHeader32 {
    () => {
        deps!();
        # [doc = " The 32-bit mach header."] # [doc = ""] # [doc = " Appears at the very beginning of the object file for 32-bit architectures."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct MachHeader32 < E : Endian > { # [doc = " mach magic number identifier"] pub magic : U32 < BigEndian > , # [doc = " cpu specifier"] pub cputype : U32 < E > , # [doc = " machine specifier"] pub cpusubtype : U32 < E > , # [doc = " type of file"] pub filetype : U32 < E > , # [doc = " number of load commands"] pub ncmds : U32 < E > , # [doc = " the size of all the load commands"] pub sizeofcmds : U32 < E > , # [doc = " flags"] pub flags : U32 < E > , }
    };
}

MachHeader32!();
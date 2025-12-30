// Generated macro for MachHeader64 (struct)
macro_rules! Depcrate_machoMachHeader64 {
() => {
// Module: crate::macho
// Provides: {"MachHeader64"}
// Dependencies: {}
# [doc = " The 64-bit mach header."] # [doc = ""] # [doc = " Appears at the very beginning of object files for 64-bit architectures."] # [derive (Debug , Clone , Copy)] # [repr (C)] pub struct MachHeader64 < E : Endian > { # [doc = " mach magic number identifier"] pub magic : U32 < BigEndian > , # [doc = " cpu specifier"] pub cputype : U32 < E > , # [doc = " machine specifier"] pub cpusubtype : U32 < E > , # [doc = " type of file"] pub filetype : U32 < E > , # [doc = " number of load commands"] pub ncmds : U32 < E > , # [doc = " the size of all the load commands"] pub sizeofcmds : U32 < E > , # [doc = " flags"] pub flags : U32 < E > , # [doc = " reserved"] pub reserved : U32 < E > , }
};
}

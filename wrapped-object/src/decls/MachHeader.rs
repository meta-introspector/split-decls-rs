macro_rules! MachHeader {
    () => {
        struct MachHeader { cputype : u32 , cpusubtype : u32 , filetype : u32 , ncmds : u32 , sizeofcmds : u32 , flags : u32 , }
    };
}

MachHeader!()
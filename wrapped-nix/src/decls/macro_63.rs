macro_rules! macro_63 {
    () => {
        cfg_if ! { if # [cfg (any (target_arch = "mips" , target_arch = "mips32r6" , target_arch = "mips64" , target_arch = "mips64r6" , target_arch = "powerpc" , target_arch = "powerpc64" , target_arch = "sparc64"))] { mod consts { # [doc (hidden)] pub const NONE : u8 = 1 ; # [doc (hidden)] pub const READ : u8 = 2 ; # [doc (hidden)] pub const WRITE : u8 = 4 ; # [doc (hidden)] pub const SIZEBITS : u8 = 13 ; # [doc (hidden)] pub const DIRBITS : u8 = 3 ; } } else { mod consts { # [doc (hidden)] pub const NONE : u8 = 0 ; # [doc (hidden)] pub const READ : u8 = 2 ; # [doc (hidden)] pub const WRITE : u8 = 1 ; # [doc (hidden)] pub const SIZEBITS : u8 = 14 ; # [doc (hidden)] pub const DIRBITS : u8 = 2 ; } } }
    };
}

macro_63!()
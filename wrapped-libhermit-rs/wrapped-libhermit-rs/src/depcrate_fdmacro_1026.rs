// Generated macro for macro_1026 (macro)
macro_rules! Depcrate_fdmacro_1026 {
() => {
// Module: crate::fd
// Provides: {"macro_1026"}
// Dependencies: {}
bitflags ! { # [derive (Debug , Copy , Clone)] pub struct AccessPermission : u32 { const S_IFMT = 0o170_000 ; const S_IFSOCK = 0o140_000 ; const S_IFLNK = 0o120_000 ; const S_IFREG = 0o100_000 ; const S_IFBLK = 0o060_000 ; const S_IFDIR = 0o040_000 ; const S_IFCHR = 0o020_000 ; const S_IFIFO = 0o010_000 ; const S_IRUSR = 0o400 ; const S_IWUSR = 0o200 ; const S_IXUSR = 0o100 ; const S_IRWXU = 0o700 ; const S_IRGRP = 0o040 ; const S_IWGRP = 0o020 ; const S_IXGRP = 0o010 ; const S_IRWXG = 0o070 ; const S_IROTH = 0o004 ; const S_IWOTH = 0o002 ; const S_IXOTH = 0o001 ; const S_IRWXO = 0o007 ; const _ = ! 0 ; } }
};
}

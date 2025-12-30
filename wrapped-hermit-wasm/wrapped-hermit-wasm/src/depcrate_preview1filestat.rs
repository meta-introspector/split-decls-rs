// Generated macro for FileStat (struct)
macro_rules! Depcrate_preview1FileStat {
() => {
// Module: crate::preview1
// Provides: {"FileStat"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , Default , IntoBytes , KnownLayout , Immutable)] # [repr (C)] pub (crate) struct FileStat { pub dev : u64 , pub ino : u64 , pub filetype : u8 , pub _pad0 : u8 , pub _pad1 : u16 , pub _pad2 : u32 , pub nlink : u64 , pub size : u64 , pub atim : u64 , pub mtim : u64 , pub ctim : u64 , }
};
}

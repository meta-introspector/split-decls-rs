// Generated macro for FdStat (struct)
macro_rules! Depcrate_preview1FdStat {
() => {
// Module: crate::preview1
// Provides: {"FdStat"}
// Dependencies: {}
# [derive (Debug , Copy , Clone , Default , IntoBytes , KnownLayout , Immutable)] # [repr (C)] pub (crate) struct FdStat { pub filetype : u8 , pub _pad0 : u8 , pub fs_flags : u16 , pub _pad1 : u32 , pub fs_rights_base : u64 , pub fs_rights_inheriting : u64 , }
};
}

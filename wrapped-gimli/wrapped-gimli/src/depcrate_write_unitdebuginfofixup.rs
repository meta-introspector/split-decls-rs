// Generated macro for DebugInfoFixup (struct)
macro_rules! Depcrate_write_unitDebugInfoFixup {
() => {
// Module: crate::write::unit
// Provides: {"DebugInfoFixup"}
// Dependencies: {}
# [doc = " A reference to a `.debug_info` entry that has yet to be resolved."] # [derive (Debug , Clone , Copy)] pub (crate) struct DebugInfoFixup { # [doc = " The offset within the section where the reference should be written."] pub offset : usize , # [doc = " The size of the reference."] pub size : u8 , # [doc = " The unit containing the entry."] pub unit : UnitId , # [doc = " The entry being referenced."] pub entry : UnitEntryId , }
};
}

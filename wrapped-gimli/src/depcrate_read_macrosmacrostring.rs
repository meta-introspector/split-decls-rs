// Generated macro for MacroString (enum)
macro_rules! Depcrate_read_macrosMacroString {
() => {
// Module: crate::read::macros
// Provides: {"MacroString"}
// Dependencies: {}
# [doc = " A string in a macro entry."] # [derive (Debug , Clone , PartialEq , Eq)] pub enum MacroString < R , Offset = < R as Reader > :: Offset > where R : Reader < Offset = Offset > , Offset : ReaderOffset , { # [doc = " The string is directly embedded in the macro entry"] Direct (R) , # [doc = " The macro refers to a string in the `.debug_str` section using a `DebugStrOffset`."] StringPointer (DebugStrOffset < Offset >) , # [doc = " The macro contains an index into an array in the `.debug_str_offsets`"] # [doc = " section, which refers to a string in the `.debug_str` section."] IndirectStringPointer (DebugStrOffsetsIndex < Offset >) , # [doc = " The macro refers to a string in the `.debug_str` section in the supplementary object file"] Supplementary (DebugStrOffset < Offset >) , }
};
}

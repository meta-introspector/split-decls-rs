// Generated macro for MacroUnitHeader (struct)
macro_rules! Depcrate_read_macrosMacroUnitHeader {
() => {
// Module: crate::read::macros
// Provides: {"MacroUnitHeader"}
// Dependencies: {}
# [derive (Debug , Clone)] struct MacroUnitHeader < R : Reader > { # [doc = " The version of the macro unit header. At the moment only version 5 is defined."] _version : u16 , flags : u8 , _debug_line_offset : DebugLineOffset < R :: Offset > , }
};
}

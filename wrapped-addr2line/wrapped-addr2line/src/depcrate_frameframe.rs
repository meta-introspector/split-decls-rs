// Generated macro for Frame (struct)
macro_rules! Depcrate_frameFrame {
() => {
// Module: crate::frame
// Provides: {"Frame"}
// Dependencies: {}
# [doc = " A function frame."] pub struct Frame < 'ctx , R : gimli :: Reader > { # [doc = " The DWARF unit offset corresponding to the DIE of the function."] pub dw_die_offset : Option < gimli :: UnitOffset < R :: Offset > > , # [doc = " The name of the function."] pub function : Option < FunctionName < R > > , # [doc = " The source location corresponding to this frame."] pub location : Option < Location < 'ctx > > , }
};
}

// Generated macro for FuncCursor (struct)
macro_rules! Depcrate_cursorFuncCursor {
() => {
// Module: crate::cursor
// Provides: {"FuncCursor"}
// Dependencies: {}
# [doc = " Function cursor."] # [doc = ""] # [doc = " A `FuncCursor` holds a mutable reference to a whole `ir::Function` while keeping a position"] # [doc = " too. The function can be re-borrowed by accessing the public `cur.func` member."] # [doc = ""] # [doc = " This cursor is for use before legalization. The inserted instructions are not given an"] # [doc = " encoding."] pub struct FuncCursor < 'f > { pos : CursorPosition , srcloc : ir :: SourceLoc , # [doc = " The referenced function."] pub func : & 'f mut ir :: Function , }
};
}

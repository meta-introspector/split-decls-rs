// Generated macro for CursorPosition (enum)
macro_rules! Depcrate_cursorCursorPosition {
() => {
// Module: crate::cursor
// Provides: {"CursorPosition"}
// Dependencies: {}
# [doc = " The possible positions of a cursor."] # [derive (Clone , Copy , PartialEq , Eq , Debug)] pub enum CursorPosition { # [doc = " Cursor is not pointing anywhere. No instructions can be inserted."] Nowhere , # [doc = " Cursor is pointing at an existing instruction."] # [doc = " New instructions will be inserted *before* the current instruction."] At (ir :: Inst) , # [doc = " Cursor is before the beginning of a block. No instructions can be inserted. Calling"] # [doc = " `next_inst()` will move to the first instruction in the block."] Before (ir :: Block) , # [doc = " Cursor is pointing after the end of a block."] # [doc = " New instructions will be appended to the block."] After (ir :: Block) , }
};
}

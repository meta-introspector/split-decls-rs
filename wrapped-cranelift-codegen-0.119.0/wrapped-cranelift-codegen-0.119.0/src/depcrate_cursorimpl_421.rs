// Generated macro for impl_421 (impl)
macro_rules! Depcrate_cursorimpl_421 {
() => {
// Module: crate::cursor
// Provides: {"impl_421"}
// Dependencies: {}
impl < 'f > Cursor for FuncCursor < 'f > { fn position (& self) -> CursorPosition { self . pos } fn set_position (& mut self , pos : CursorPosition) { self . pos = pos } fn srcloc (& self) -> ir :: SourceLoc { self . srcloc } fn set_srcloc (& mut self , srcloc : ir :: SourceLoc) { self . func . params . ensure_base_srcloc (srcloc) ; self . srcloc = srcloc ; } fn layout (& self) -> & ir :: Layout { & self . func . layout } fn layout_mut (& mut self) -> & mut ir :: Layout { & mut self . func . layout } }
};
}

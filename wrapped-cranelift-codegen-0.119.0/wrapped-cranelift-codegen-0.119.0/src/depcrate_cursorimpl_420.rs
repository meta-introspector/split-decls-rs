// Generated macro for impl_420 (impl)
macro_rules! Depcrate_cursorimpl_420 {
() => {
// Module: crate::cursor
// Provides: {"impl_420"}
// Dependencies: {}
impl < 'f > FuncCursor < 'f > { # [doc = " Create a new `FuncCursor` pointing nowhere."] pub fn new (func : & 'f mut ir :: Function) -> Self { Self { pos : CursorPosition :: Nowhere , srcloc : Default :: default () , func , } } # [doc = " Use the source location of `inst` for future instructions."] pub fn use_srcloc (& mut self , inst : ir :: Inst) { self . srcloc = self . func . srcloc (inst) ; } # [doc = " Create an instruction builder that inserts an instruction at the current position."] pub fn ins (& mut self) -> ir :: InsertBuilder < '_ , & mut FuncCursor < 'f > > { ir :: InsertBuilder :: new (self) } }
};
}

// Generated macro for InitializeVisitorState (enum)
macro_rules! Depcrate_loops_utilsInitializeVisitorState {
() => {
// Module: crate::loops::utils
// Provides: {"InitializeVisitorState"}
// Dependencies: {}
enum InitializeVisitorState < 'hir > { Initial , Declared (Symbol , Option < Ty < 'hir > >) , Initialized { name : Symbol , ty : Option < Ty < 'hir > > , initializer : & 'hir Expr < 'hir > , } , DontWarn , }
};
}

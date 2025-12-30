// Generated macro for path_kind_len (function)
macro_rules! Depcrate_find_pathpath_kind_len {
() => {
// Module: crate::find_path
// Provides: {"path_kind_len"}
// Dependencies: {}
fn path_kind_len (kind : PathKind) -> usize { match kind { PathKind :: Plain => 0 , PathKind :: Super (0) => 4 , PathKind :: Super (s) => s as usize * 5 , PathKind :: Crate => 5 , PathKind :: Abs => 2 , PathKind :: DollarCrate (_) => 0 , } }
};
}

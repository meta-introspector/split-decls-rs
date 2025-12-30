// Generated macro for impl_972 (impl)
macro_rules! Depcrate_iter_positionsimpl_972 {
() => {
// Module: crate::iter::positions
// Provides: {"impl_972"}
// Dependencies: {}
impl < F , P , T > Folder < T > for PositionsFolder < '_ , F , P > where F : Folder < usize > , P : Fn (T) -> bool , { type Result = F :: Result ; fn consume (mut self , item : T) -> Self { let index = self . offset ; self . offset += 1 ; if (self . predicate) (item) { self . base = self . base . consume (index) ; } self } fn complete (self) -> Self :: Result { self . base . complete () } fn full (& self) -> bool { self . base . full () } }
};
}

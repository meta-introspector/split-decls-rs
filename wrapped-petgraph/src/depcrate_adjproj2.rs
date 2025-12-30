// Generated macro for proj2 (function)
macro_rules! Depcrate_adjproj2 {
() => {
// Module: crate::adj
// Provides: {"proj2"}
// Dependencies: {}
fn proj2 < E , Ix : IndexType > ((row_index , row) : (usize , & Vec < WSuc < E , Ix > >)) -> SomeIter < '_ , E , Ix > { row . iter () . enumerate () . zip (core :: iter :: repeat (Ix :: new (row_index))) . map (proj1 as _) }
};
}

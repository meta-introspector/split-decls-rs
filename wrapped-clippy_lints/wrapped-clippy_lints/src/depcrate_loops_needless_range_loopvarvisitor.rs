// Generated macro for VarVisitor (struct)
macro_rules! Depcrate_loops_needless_range_loopVarVisitor {
() => {
// Module: crate::loops::needless_range_loop
// Provides: {"VarVisitor"}
// Dependencies: {}
# [expect (clippy :: struct_excessive_bools)] struct VarVisitor < 'a , 'tcx > { # [doc = " context reference"] cx : & 'a LateContext < 'tcx > , # [doc = " var name to look for as index"] var : HirId , # [doc = " indexed variables that are used mutably"] indexed_mut : FxHashSet < Symbol > , # [doc = " indirectly indexed variables (`v[(i + 4) % N]`), the extend is `None` for global"] indexed_indirectly : FxHashMap < Symbol , Option < region :: Scope > > , # [doc = " indirectly indexed literals, like `[1, 2, 3][(i + 4) % N]`"] unnamed_indexed_indirectly : bool , # [doc = " subset of `indexed` of vars that are indexed directly: `v[i]`"] # [doc = " this will not contain cases like `v[calc_index(i)]` or `v[(i + 4) % N]`"] indexed_directly : FxIndexMap < Symbol , (Option < region :: Scope > , Ty < 'tcx >) > , # [doc = " directly indexed literals, like `[1, 2, 3][i]`"] unnamed_indexed_directly : bool , # [doc = " Any names that are used outside an index operation."] # [doc = " Used to detect things like `&mut vec` used together with `vec[i]`"] referenced : FxHashSet < Symbol > , # [doc = " has the loop variable been used in expressions other than the index of"] # [doc = " an index op?"] nonindex : bool , # [doc = " Whether we are inside the `$` in `&mut $` or `$ = foo` or `$.bar`, where bar"] # [doc = " takes `&mut self`"] prefer_mutable : bool , }
};
}

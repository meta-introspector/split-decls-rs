// Generated macro for IndexConst (struct)
macro_rules! Depcrate_curve25519_solinas_64IndexConst {
() => {
// Module: crate::curve25519_solinas_64
// Provides: {"IndexConst"}
// Dependencies: {}
# [doc = " Since `Index` and `IndexMut` aren't callable in `const` contexts yet, this helper type helps unify"] # [doc = " arrays and user-defined array-wrapper types into a single type which can be indexed in `const`"] # [doc = " contexts. Once `const trait`s are stabilized this type can go away"] struct IndexConst < T : ? Sized > (T) ;
};
}

// Generated macro for impl_39 (impl)
macro_rules! Depcrate_ustrimpl_39 {
() => {
// Module: crate::ustr
// Provides: {"impl_39"}
// Dependencies: {}
# [doc = " This impl requires enabling the optional `zerovec` Cargo feature"] # [cfg (all (feature = "zerovec" , feature = "alloc"))] impl < 'a > zerovec :: maps :: ZeroMapKV < 'a > for PotentialUtf8 { type Container = zerovec :: VarZeroVec < 'a , PotentialUtf8 > ; type Slice = zerovec :: VarZeroSlice < PotentialUtf8 > ; type GetType = PotentialUtf8 ; type OwnedType = Box < PotentialUtf8 > ; }
};
}

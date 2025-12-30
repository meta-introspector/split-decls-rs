// Generated macro for impl_74 (impl)
macro_rules! Depcrate_frontend_zerovecimpl_74 {
() => {
// Module: crate::frontend::zerovec
// Provides: {"impl_74"}
// Dependencies: {}
impl < 'a , B : PatternBackend > ZeroMapKV < 'a > for Pattern < B > where Pattern < B > : VarULE , { type Container = VarZeroVec < 'a , Pattern < B > > ; type Slice = VarZeroSlice < Pattern < B > > ; type GetType = Pattern < B > ; type OwnedType = Box < Pattern < B > > ; }
};
}

// Generated macro for impl_80 (impl)
macro_rules! Depcrate_features_rkyvimpl_80 {
() => {
// Module: crate::features::rkyv
// Provides: {"impl_80"}
// Dependencies: {}
impl Archive for CompactString { type Archived = ArchivedString ; type Resolver = StringResolver ; # [inline] fn resolve (& self , resolver : Self :: Resolver , out : Place < Self :: Archived >) { ArchivedString :: resolve_from_str (self . as_str () , resolver , out) ; } }
};
}

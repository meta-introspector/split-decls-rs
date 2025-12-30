// Generated macro for impl_727 (impl)
macro_rules! Depcrate_util_capturesimpl_727 {
() => {
// Module: crate::util::captures
// Provides: {"impl_727"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for GroupInfoError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self . kind { GroupInfoErrorKind :: TooManyPatterns { .. } | GroupInfoErrorKind :: TooManyGroups { .. } | GroupInfoErrorKind :: MissingGroups { .. } | GroupInfoErrorKind :: FirstMustBeUnnamed { .. } | GroupInfoErrorKind :: Duplicate { .. } => None , } } }
};
}

// Generated macro for impl_724 (impl)
macro_rules! Depcrate_util_capturesimpl_724 {
() => {
// Module: crate::util::captures
// Provides: {"impl_724"}
// Dependencies: {}
# [cfg (feature = "std")] impl std :: error :: Error for GroupInfoError { fn source (& self) -> Option < & (dyn std :: error :: Error + 'static) > { match self . kind { GroupInfoErrorKind :: TooManyPatterns { .. } | GroupInfoErrorKind :: TooManyGroups { .. } | GroupInfoErrorKind :: MissingGroups { .. } | GroupInfoErrorKind :: FirstMustBeUnnamed { .. } | GroupInfoErrorKind :: Duplicate { .. } => None , } } }
};
}

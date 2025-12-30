// Generated macro for NonCanonicalImpls (struct)
macro_rules! Depcrate_non_canonical_implsNonCanonicalImpls {
() => {
// Module: crate::non_canonical_impls
// Provides: {"NonCanonicalImpls"}
// Dependencies: {}
# [expect (clippy :: struct_field_names , reason = "`_trait` suffix is meaningful on its own, \
              and creating an inner `StoredTraits` struct would just add a level of indirection")] pub (crate) struct NonCanonicalImpls { partial_ord_trait : Option < DefId > , ord_trait : Option < DefId > , clone_trait : Option < DefId > , copy_trait : Option < DefId > , }
};
}

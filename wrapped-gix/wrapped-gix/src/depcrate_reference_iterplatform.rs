// Generated macro for Platform (struct)
macro_rules! Depcrate_reference_iterPlatform {
() => {
// Module: crate::reference::iter
// Provides: {"Platform"}
// Dependencies: {}
# [doc = " A platform to create iterators over references."] # [must_use = "Iterators should be obtained from this iterator platform"] pub struct Platform < 'r > { pub (crate) platform : gix_ref :: file :: iter :: Platform < 'r > , # [doc = " The owning repository."] pub repo : & 'r crate :: Repository , }
};
}

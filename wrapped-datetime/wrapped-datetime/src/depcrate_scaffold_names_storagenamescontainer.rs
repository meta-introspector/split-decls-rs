// Generated macro for NamesContainer (trait)
macro_rules! Depcrate_scaffold_names_storageNamesContainer {
() => {
// Module: crate::scaffold::names_storage
// Provides: {"NamesContainer"}
// Dependencies: {}
# [doc = " Trait that associates a container for a payload parameterized by the given variables."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. Do not implement this trait in userland unless you are prepared for things to occasionally break."] # [doc = " </div>"] # [allow (missing_docs)] pub trait NamesContainer < M : DynamicDataMarker , Variables > : UnstableSealed where Variables : PartialEq + Copy + fmt :: Debug , { type Container : MaybePayload < M , Variables > + fmt :: Debug + Clone ; }
};
}

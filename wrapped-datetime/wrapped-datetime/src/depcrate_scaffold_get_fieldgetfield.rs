// Generated macro for GetField (trait)
macro_rules! Depcrate_scaffold_get_fieldGetField {
() => {
// Module: crate::scaffold::get_field
// Provides: {"GetField"}
// Dependencies: {}
# [doc = " A type that can return a certain field `T`."] # [doc = ""] # [doc = " This is used as a bound on various datetime functions."] # [doc = ""] # [doc = " <div class=\"stab unstable\">"] # [doc = " 🚧 This trait is considered unstable; it may change at any time, in breaking or non-breaking ways,"] # [doc = " including in SemVer minor releases. Do not implement this trait in userland unless you are prepared for things to occasionally break."] # [doc = " </div>"] pub trait GetField < T > : UnstableSealed { # [doc = " Returns the value of this trait's field `T`."] fn get_field (& self) -> T ; }
};
}

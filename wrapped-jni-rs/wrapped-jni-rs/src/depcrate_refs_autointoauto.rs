// Generated macro for IntoAuto (trait)
macro_rules! Depcrate_refs_autoIntoAuto {
() => {
// Module: crate::refs::auto
// Provides: {"IntoAuto"}
// Dependencies: {}
# [doc = " A trait for wrapping a local reference type into an [`Auto`]"] pub trait IntoAuto < 'local > : Sized + Into < JObject < 'local > > { # [doc = " Wraps the local reference type into an auto-delete [`Auto`] that will"] # [doc = " automatically delete the local reference when it is dropped"] fn auto (self) -> Auto < 'local , Self > { Auto :: new (self) } }
};
}

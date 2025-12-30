// Generated macro for Access (trait)
macro_rules! Depcrate_accessAccess {
() => {
// Module: crate::access
// Provides: {"Access"}
// Dependencies: {}
# [doc = " Abstracts over ways code can get access to a value of type `T`."] # [doc = ""] # [doc = " This is the trait that parts of code will use when accessing a subpart of the big data"] # [doc = " structure. See the [module documentation](index.html) for details."] pub trait Access < T > { # [doc = " A guard object containing the value and keeping it alive."] # [doc = ""] # [doc = " For technical reasons, the library doesn't allow direct access into the stored value. A"] # [doc = " temporary guard object must be loaded, that keeps the actual value alive for the time of"] # [doc = " use."] type Guard : Deref < Target = T > ; # [doc = " The loading method."] # [doc = ""] # [doc = " This returns the guard that holds the actual value. Should be called anew each time a fresh"] # [doc = " value is needed."] fn load (& self) -> Self :: Guard ; }
};
}

// Generated macro for Submodule (struct)
macro_rules! Depcrate_typesSubmodule {
() => {
// Module: crate::types
// Provides: {"Submodule"}
// Dependencies: {}
# [doc = " A stand-in for the submodule of a particular name."] # [derive (Clone)] # [cfg (feature = "attributes")] pub struct Submodule < 'repo > { pub (crate) state : std :: rc :: Rc < crate :: submodule :: SharedState < 'repo > > , pub (crate) name : crate :: bstr :: BString , }
};
}

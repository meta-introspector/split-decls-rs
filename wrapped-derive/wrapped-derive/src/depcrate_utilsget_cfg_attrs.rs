// Generated macro for get_cfg_attrs (function)
macro_rules! Depcrate_utilsget_cfg_attrs {
() => {
// Module: crate::utils
// Provides: {"get_cfg_attrs"}
// Dependencies: {}
pub fn get_cfg_attrs (attrs : & [Attribute]) -> Vec < Attribute > { attrs . iter () . filter (| attr | ! attr . path () . segments . is_empty () && attr . path () . segments [0] . ident == "cfg") . cloned () . collect () }
};
}

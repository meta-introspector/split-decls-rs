// Generated macro for add_cfg_attrs_to (function)
macro_rules! Depcrate_utilsadd_cfg_attrs_to {
() => {
// Module: crate::utils
// Provides: {"add_cfg_attrs_to"}
// Dependencies: {}
pub (crate) fn add_cfg_attrs_to < T , U > (from : & T , to : & U) where T : HasAttrs , U : AttrsOwnerEdit , { let cfg_attrs = from . attrs () . filter (| attr | attr . as_simple_call () . is_some_and (| (name , _arg) | name == "cfg")) ; for attr in cfg_attrs { to . add_attr (attr . clone_for_update ()) ; } }
};
}

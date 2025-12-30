// Generated macro for bindgen (module)
macro_rules! Depcratebindgen {
() => {
// Module: crate
// Provides: {"bindgen"}
// Dependencies: {}
mod bindgen { # [cfg (not (bindgen_rs_file))] include ! (concat ! (env ! ("OUT_DIR") , "/bindgen.rs")) ; # [cfg (bindgen_rs_file)] include ! (env ! ("BINDGEN_RS_FILE")) ; }
};
}

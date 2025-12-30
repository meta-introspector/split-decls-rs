// Generated macro for add_post_link_args (function)
macro_rules! Depcrate_back_linkadd_post_link_args {
() => {
// Module: crate::back::link
// Provides: {"add_post_link_args"}
// Dependencies: {}
# [doc = " Add arbitrary \"post-link\" args defined by the target spec."] # [doc = " FIXME: Determine where exactly these args need to be inserted."] fn add_post_link_args (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor) { if let Some (args) = sess . target . post_link_args . get (& flavor) { cmd . verbatim_args (args . iter () . map (Deref :: deref)) ; } }
};
}

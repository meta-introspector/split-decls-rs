// Generated macro for add_pre_link_args (function)
macro_rules! Depcrate_back_linkadd_pre_link_args {
() => {
// Module: crate::back::link
// Provides: {"add_pre_link_args"}
// Dependencies: {}
# [doc = " Add arbitrary \"pre-link\" args defined by the target spec or from command line."] # [doc = " FIXME: Determine where exactly these args need to be inserted."] fn add_pre_link_args (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor) { if let Some (args) = sess . target . pre_link_args . get (& flavor) { cmd . verbatim_args (args . iter () . map (Deref :: deref)) ; } cmd . verbatim_args (& sess . opts . unstable_opts . pre_link_args) ; }
};
}

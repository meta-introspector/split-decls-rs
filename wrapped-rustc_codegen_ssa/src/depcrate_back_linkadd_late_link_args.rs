// Generated macro for add_late_link_args (function)
macro_rules! Depcrate_back_linkadd_late_link_args {
() => {
// Module: crate::back::link
// Provides: {"add_late_link_args"}
// Dependencies: {}
# [doc = " Add arbitrary \"late link\" args defined by the target spec."] # [doc = " FIXME: Determine where exactly these args need to be inserted."] fn add_late_link_args (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor , crate_type : CrateType , codegen_results : & CodegenResults ,) { let any_dynamic_crate = crate_type == CrateType :: Dylib || crate_type == CrateType :: Sdylib || codegen_results . crate_info . dependency_formats . iter () . any (| (ty , list) | { * ty == crate_type && list . iter () . any (| & linkage | linkage == Linkage :: Dynamic) }) ; if any_dynamic_crate { if let Some (args) = sess . target . late_link_args_dynamic . get (& flavor) { cmd . verbatim_args (args . iter () . map (Deref :: deref)) ; } } else if let Some (args) = sess . target . late_link_args_static . get (& flavor) { cmd . verbatim_args (args . iter () . map (Deref :: deref)) ; } if let Some (args) = sess . target . late_link_args . get (& flavor) { cmd . verbatim_args (args . iter () . map (Deref :: deref)) ; } }
};
}

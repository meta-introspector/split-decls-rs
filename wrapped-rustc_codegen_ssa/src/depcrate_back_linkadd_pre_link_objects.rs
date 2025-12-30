// Generated macro for add_pre_link_objects (function)
macro_rules! Depcrate_back_linkadd_pre_link_objects {
() => {
// Module: crate::back::link
// Provides: {"add_pre_link_objects"}
// Dependencies: {}
# [doc = " Add pre-link object files defined by the target spec."] fn add_pre_link_objects (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor , link_output_kind : LinkOutputKind , self_contained : bool ,) { let opts = & sess . target ; let empty = Default :: default () ; let objects = if self_contained { & opts . pre_link_objects_self_contained } else if ! (sess . target . os == "fuchsia" && matches ! (flavor , LinkerFlavor :: Gnu (Cc :: Yes , _))) { & opts . pre_link_objects } else { & empty } ; for obj in objects . get (& link_output_kind) . iter () . copied () . flatten () { cmd . add_object (& get_object_file_path (sess , obj , self_contained)) ; } }
};
}

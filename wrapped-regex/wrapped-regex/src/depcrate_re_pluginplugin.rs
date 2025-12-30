// Generated macro for Plugin (struct)
macro_rules! Depcrate_re_pluginPlugin {
() => {
// Module: crate::re_plugin
// Provides: {"Plugin"}
// Dependencies: {}
# [doc = " Plugin is the compiler plugin's data structure. It declare some static"] # [doc = " data (like capture groups and the original regex string), but defines its"] # [doc = " matching engine as a simple function."] # [doc (hidden)] pub struct Plugin { # [doc (hidden)] pub original : & 'static str , # [doc (hidden)] pub names : & 'static & 'static [Option < & 'static str >] , # [doc (hidden)] pub groups : & 'static & 'static [(& 'static str , usize)] , # [doc (hidden)] pub prog : fn (& mut [Slot] , & str , usize) -> bool , }
};
}

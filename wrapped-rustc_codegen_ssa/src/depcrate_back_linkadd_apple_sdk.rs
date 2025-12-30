// Generated macro for add_apple_sdk (function)
macro_rules! Depcrate_back_linkadd_apple_sdk {
() => {
// Module: crate::back::link
// Provides: {"add_apple_sdk"}
// Dependencies: {}
fn add_apple_sdk (cmd : & mut dyn Linker , sess : & Session , flavor : LinkerFlavor) -> Option < PathBuf > { if ! sess . target . is_like_darwin { return None ; } let LinkerFlavor :: Darwin (cc , _) = flavor else { return None ; } ; let sdkroot = sess . time ("get_apple_sdk_root" , | | get_apple_sdk_root (sess)) ? ; if cc == Cc :: Yes { cmd . cmd () . env ("SDKROOT" , & sdkroot) ; } else { cmd . link_arg ("-syslibroot") ; cmd . link_arg (& sdkroot) ; } Some (sdkroot) }
};
}

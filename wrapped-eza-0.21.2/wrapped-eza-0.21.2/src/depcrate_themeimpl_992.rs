// Generated macro for impl_992 (impl)
macro_rules! Depcrate_themeimpl_992 {
() => {
// Module: crate::theme
// Provides: {"impl_992"}
// Dependencies: {}
impl Definitions { # [doc = " Parse the environment variables into `LS_COLORS` pairs, putting file glob"] # [doc = " colours into the `ExtensionMappings` that gets returned, and using the"] # [doc = " two-character UI codes to modify the mutable `Colours`."] # [doc = ""] # [doc = " Also returns if the `EZA_COLORS` variable should reset the existing file"] # [doc = " type mappings or not. The `reset` code needs to be the first one."] fn parse_color_vars (& self , colours : & mut UiStyles) -> (ExtensionMappings , bool) { use log :: * ; let mut exts = ExtensionMappings :: default () ; if let Some (lsc) = & self . ls { LSColors (lsc) . each_pair (| pair | { if ! colours . set_ls (& pair) { match glob :: Pattern :: new (pair . key) { Ok (pat) => { exts . add (pat , pair . to_style ()) ; } Err (e) => { warn ! ("Couldn't parse glob pattern {:?}: {}" , pair . key , e) ; } } } }) ; } let mut use_default_filetypes = true ; if let Some (exa) = & self . exa { if exa == "reset" || exa . starts_with ("reset:") { use_default_filetypes = false ; } LSColors (exa) . each_pair (| pair | { if ! colours . set_ls (& pair) && ! colours . set_exa (& pair) { match glob :: Pattern :: new (pair . key) { Ok (pat) => { exts . add (pat , pair . to_style ()) ; } Err (e) => { warn ! ("Couldn't parse glob pattern {:?}: {}" , pair . key , e) ; } } } ; }) ; } (exts , use_default_filetypes) } }
};
}

// Generated macro for DEFMT_VERSION (static)
macro_rules! DepcrateDEFMT_VERSION {
() => {
// Module: crate
// Provides: {"DEFMT_VERSION"}
// Dependencies: {}
# [doc = " The defmt ABI and wire format version."] # [doc = ""] # [doc = " This number has to be updated every time there is a backwards-incompatible change to"] # [doc = " - the symbol naming scheme"] # [doc = " - the symbol and section layout"] # [doc = " - the data encoding / wire format"] # [used] # [cfg_attr (target_os = "macos" , link_section = ".defmt,end.VERSION")] # [cfg_attr (not (target_os = "macos") , link_section = ".defmt.end")] # [export_name = "_defmt_version_ = 4"] static DEFMT_VERSION : u8 = 0 ;
};
}

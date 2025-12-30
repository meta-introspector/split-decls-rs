// Generated macro for SplitDeclsConfig (struct)
macro_rules! DepcrateSplitDeclsConfig {
() => {
// Module: crate
// Provides: {"SplitDeclsConfig"}
// Dependencies: {}
# [derive (Debug , Clone , Default)] pub struct SplitDeclsConfig { pub patches : Option < HashMap < String , Vec < String > > > , pub string_replacements : Option < Vec < StringReplacement > > , pub custom_prelude_overlay : Option < String > , pub active_overlay_modules : Vec < String > , pub crates_io_patches : HashMap < String , String > , }
};
}

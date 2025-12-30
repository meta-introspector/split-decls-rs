// Generated macro for ImageDynamicRelocation64V2 (struct)
macro_rules! Depcrate_peImageDynamicRelocation64V2 {
() => {
// Module: crate::pe
// Provides: {"ImageDynamicRelocation64V2"}
// Dependencies: {}
# [derive (Debug , Clone , Copy)] # [repr (C)] pub struct ImageDynamicRelocation64V2 { pub header_size : U32 < LE > , pub fixup_info_size : U32 < LE > , pub symbol : U64 < LE > , pub symbol_group : U32 < LE > , pub flags : U32 < LE > , }
};
}

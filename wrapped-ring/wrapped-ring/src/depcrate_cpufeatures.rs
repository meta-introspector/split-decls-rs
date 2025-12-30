// Generated macro for features (module)
macro_rules! Depcrate_cpufeatures {
() => {
// Module: crate::cpu
// Provides: {"features"}
// Dependencies: {}
mod features { use crate :: polyfill :: NotSend ; # [doc = " A witness indicating that CPU features have been detected and cached."] # [doc = ""] # [doc = " This is a zero-sized type so that it can be \"stored\" wherever convenient."] # [derive (Copy , Clone)] pub (crate) struct Features (NotSend) ; impl Features { pub fn values (self) -> Values { Values { values : super :: featureflags :: get (self) , cpu : self , } } } cfg_if :: cfg_if ! { if # [cfg (any (all (target_arch = "aarch64" , target_endian = "little") , all (target_arch = "arm" , target_endian = "little") , target_arch = "x86" , target_arch = "x86_64"))] { impl Features { pub (super) unsafe fn new_after_feature_flags_written_and_synced_unchecked () -> Self { Self (NotSend :: VALUE) } } } else { impl Features { pub (super) fn new_no_features_to_detect () -> Self { Self (NotSend :: VALUE) } } } } pub struct Values { values : u32 , cpu : Features , } impl Values { # [inline (always)] pub (super) fn values (& self) -> u32 { self . values } # [inline (always)] pub (super) fn cpu (& self) -> Features { self . cpu } } }
};
}

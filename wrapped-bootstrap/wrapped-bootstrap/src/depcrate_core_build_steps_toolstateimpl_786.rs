// Generated macro for impl_786 (impl)
macro_rules! Depcrate_core_build_steps_toolstateimpl_786 {
() => {
// Module: crate::core::build_steps::toolstate
// Provides: {"impl_786"}
// Dependencies: {}
impl Builder < '_ > { fn toolstates (& self) -> HashMap < Box < str > , ToolState > { if let Some (ref path) = self . config . save_toolstates { if let Some (parent) = path . parent () { t ! (std :: fs :: create_dir_all (parent)) ; } let mut file = t ! (fs :: OpenOptions :: new () . create (true) . truncate (false) . write (true) . read (true) . open (path)) ; serde_json :: from_reader (& mut file) . unwrap_or_default () } else { Default :: default () } } # [doc = " Updates the actual toolstate of a tool."] # [doc = ""] # [doc = " The toolstates are saved to the file specified by the key"] # [doc = " `rust.save-toolstates` in `bootstrap.toml`. If unspecified, nothing will be"] # [doc = " done. The file is updated immediately after this function completes."] pub fn save_toolstate (& self , tool : & str , state : ToolState) { use std :: io :: Write ; if self . config . dry_run () { return ; } if tool == "clippy-driver" || tool == "rustfmt" { return ; } if let Some (ref path) = self . config . save_toolstates { if let Some (parent) = path . parent () { t ! (std :: fs :: create_dir_all (parent)) ; } let mut file = t ! (fs :: OpenOptions :: new () . create (true) . truncate (false) . read (true) . write (true) . open (path)) ; let mut current_toolstates : HashMap < Box < str > , ToolState > = serde_json :: from_reader (& mut file) . unwrap_or_default () ; current_toolstates . insert (tool . into () , state) ; t ! (file . seek (SeekFrom :: Start (0))) ; t ! (file . set_len (0)) ; t ! (serde_json :: to_writer (& file , & current_toolstates)) ; t ! (writeln ! (file)) ; } } }
};
}

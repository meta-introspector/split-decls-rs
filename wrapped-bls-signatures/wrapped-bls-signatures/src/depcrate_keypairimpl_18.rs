// Generated macro for impl_18 (impl)
macro_rules! Depcrate_keypairimpl_18 {
() => {
// Module: crate::keypair
// Provides: {"impl_18"}
// Dependencies: {}
# [cfg (feature = "std")] impl Keypair { pub fn read_json < R : Read > (reader : & mut R) -> Result < Self , Box < dyn error :: Error > > { let bytes : Vec < u8 > = serde_json :: from_reader (reader) ? ; Self :: try_from (bytes . as_slice ()) . ok () . ok_or_else (| | std :: io :: Error :: other ("Invalid BLS keypair") . into ()) } pub fn read_json_file < F : AsRef < Path > > (path : F) -> Result < Self , Box < dyn error :: Error > > { let mut file = File :: open (path . as_ref ()) ? ; Self :: read_json (& mut file) } pub fn write_json < W : Write > (& self , writer : & mut W) -> Result < String , Box < dyn error :: Error > > { let json = serde_json :: to_string (& Into :: < [u8 ; BLS_KEYPAIR_SIZE] > :: into (self) . as_slice ()) ? ; writer . write_all (& json . clone () . into_bytes ()) ? ; Ok (json) } pub fn write_json_file < F : AsRef < Path > > (& self , outfile : F ,) -> Result < String , Box < dyn core :: error :: Error > > { let outfile = outfile . as_ref () ; if let Some (outdir) = outfile . parent () { fs :: create_dir_all (outdir) ? ; } let mut f = { # [cfg (not (unix))] { OpenOptions :: new () } # [cfg (unix)] { use std :: os :: unix :: fs :: OpenOptionsExt ; OpenOptions :: new () . mode (0o600) } } . write (true) . truncate (true) . create (true) . open (outfile) ? ; self . write_json (& mut f) } }
};
}

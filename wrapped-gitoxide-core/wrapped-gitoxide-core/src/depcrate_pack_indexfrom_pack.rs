// Generated macro for from_pack (function)
macro_rules! Depcrate_pack_indexfrom_pack {
() => {
// Module: crate::pack::index
// Provides: {"from_pack"}
// Dependencies: {}
pub fn from_pack (pack : PathOrRead , directory : Option < PathBuf > , mut progress : impl NestedProgress + 'static , ctx : Context < 'static , impl io :: Write > ,) -> anyhow :: Result < () > { use anyhow :: Context ; let options = pack :: bundle :: write :: Options { thread_limit : ctx . thread_limit , iteration_mode : ctx . iteration_mode . into () , index_version : pack :: index :: Version :: default () , object_hash : ctx . object_hash , } ; let out = ctx . out ; let format = ctx . format ; let res = match pack { PathOrRead :: Path (pack) => { let pack_len = pack . metadata () ? . len () ; let pack_file = fs :: File :: open (pack) ? ; pack :: Bundle :: write_to_directory_eagerly (Box :: new (pack_file) , Some (pack_len) , directory , & mut progress , ctx . should_interrupt , None :: < gix :: objs :: find :: Never > , options ,) } PathOrRead :: Read (input) => pack :: Bundle :: write_to_directory_eagerly (input , None , directory , & mut progress , ctx . should_interrupt , None :: < gix :: objs :: find :: Never > , options ,) , } . with_context (| | "Failed to write pack and index") ? ; match format { OutputFormat :: Human => drop (human_output (out , res)) , # [cfg (feature = "serde")] OutputFormat :: Json => serde_json :: to_writer_pretty (out , & res) ? , } Ok (()) }
};
}

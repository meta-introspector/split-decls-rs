// Generated macro for info (function)
macro_rules! Depcrate_repository_odbinfo {
() => {
// Module: crate::repository::odb
// Provides: {"info"}
// Dependencies: {}
# [cfg_attr (not (feature = "serde") , allow (unused_variables))] pub fn info (repo : gix :: Repository , format : OutputFormat , out : impl io :: Write , mut err : impl io :: Write ,) -> anyhow :: Result < () > { if format == OutputFormat :: Human { writeln ! (err , "Only JSON is implemented - using that instead") ? ; } # [cfg_attr (feature = "serde" , derive (serde :: Serialize))] pub struct Statistics { # [cfg_attr (not (feature = "serde") , allow (dead_code))] pub path : std :: path :: PathBuf , # [cfg_attr (not (feature = "serde") , allow (dead_code))] pub object_hash : String , # [cfg_attr (not (feature = "serde") , allow (dead_code))] pub use_multi_pack_index : bool , # [cfg_attr (not (feature = "serde") , allow (dead_code))] pub structure : Vec < gix :: odb :: store :: structure :: Record > , # [cfg_attr (not (feature = "serde") , allow (dead_code))] pub metrics : gix :: odb :: store :: Metrics , } let store = repo . objects . store_ref () ; let stats = Statistics { path : store . path () . into () , object_hash : store . object_hash () . to_string () , use_multi_pack_index : store . use_multi_pack_index () , structure : store . structure () ? , metrics : store . metrics () , } ; # [cfg (feature = "serde")] { serde_json :: to_writer_pretty (out , & stats) ? ; } Ok (()) }
};
}

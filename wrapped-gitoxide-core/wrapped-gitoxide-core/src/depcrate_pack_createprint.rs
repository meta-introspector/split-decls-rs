// Generated macro for print (function)
macro_rules! Depcrate_pack_createprint {
() => {
// Module: crate::pack::create
// Provides: {"print"}
// Dependencies: {}
fn print (stats : Statistics , format : OutputFormat , out : impl std :: io :: Write) -> anyhow :: Result < () > { match format { OutputFormat :: Human => human_output (stats , out) . map_err (Into :: into) , # [cfg (feature = "serde")] OutputFormat :: Json => serde_json :: to_writer_pretty (out , & stats) . map_err (Into :: into) , } }
};
}

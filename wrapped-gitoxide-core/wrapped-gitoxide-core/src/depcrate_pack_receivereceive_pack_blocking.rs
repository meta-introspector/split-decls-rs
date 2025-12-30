// Generated macro for receive_pack_blocking (function)
macro_rules! Depcrate_pack_receivereceive_pack_blocking {
() => {
// Module: crate::pack::receive
// Provides: {"receive_pack_blocking"}
// Dependencies: {}
# [allow (clippy :: too_many_arguments)] fn receive_pack_blocking (mut directory : Option < PathBuf > , mut refs_directory : Option < PathBuf > , mut input : impl io :: BufRead , progress : & mut dyn DynNestedProgress , refs : & [Ref] , should_interrupt : & AtomicBool , mut out : impl std :: io :: Write , thread_limit : Option < usize > , object_hash : gix :: hash :: Kind , format : OutputFormat ,) -> io :: Result < () > { let options = pack :: bundle :: write :: Options { thread_limit , index_version : pack :: index :: Version :: V2 , iteration_mode : pack :: data :: input :: Mode :: Verify , object_hash , } ; let outcome = pack :: Bundle :: write_to_directory (& mut input , directory . take () . as_deref () , progress , should_interrupt , None :: < gix :: objs :: find :: Never > , options ,) . map_err (io :: Error :: other) ? ; if let Some (directory) = refs_directory . take () { write_raw_refs (refs , directory) ? ; } match format { OutputFormat :: Human => drop (print (& mut out , outcome , refs)) , # [cfg (feature = "serde")] OutputFormat :: Json => { serde_json :: to_writer_pretty (& mut out , & JsonOutcome :: from_outcome_and_refs (outcome , refs)) ? ; } } Ok (()) }
};
}

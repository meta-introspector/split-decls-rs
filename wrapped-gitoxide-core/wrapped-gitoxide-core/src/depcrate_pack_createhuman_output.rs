// Generated macro for human_output (function)
macro_rules! Depcrate_pack_createhuman_output {
() => {
// Module: crate::pack::create
// Provides: {"human_output"}
// Dependencies: {}
fn human_output (Statistics { counts : pack :: data :: output :: count :: objects :: Outcome { input_objects , expanded_objects , decoded_objects , total_objects , } , entries : pack :: data :: output :: entry :: iter_from_counts :: Outcome { decoded_and_recompressed_objects , missing_objects , objects_copied_from_pack , ref_delta_objects , } , } : Statistics , mut out : impl std :: io :: Write ,) -> std :: io :: Result < () > { let width = 30 ; writeln ! (out , "counting phase") ? ; # [rustfmt :: skip] writeln ! (out , "\t{:<width$} {}\n\t{:<width$} {}\n\t{:<width$} {}\n\t{:<width$} {}" , "input objects" , input_objects , "expanded objects" , expanded_objects , "decoded objects" , decoded_objects , "total objects" , total_objects , width = width) ? ; writeln ! (out , "generation phase") ? ; # [rustfmt :: skip] writeln ! (out , "\t{:<width$} {}\n\t{:<width$} {}\n\t{:<width$} {}\n\t{:<width$} {}" , "decoded and recompressed" , decoded_and_recompressed_objects , "pack-to-pack copies" , objects_copied_from_pack , "ref-delta-objects" , ref_delta_objects , "missing objects" , missing_objects , width = width) ? ; Ok (()) }
};
}

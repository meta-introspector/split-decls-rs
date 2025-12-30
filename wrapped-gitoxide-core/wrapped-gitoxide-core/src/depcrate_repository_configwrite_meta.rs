// Generated macro for write_meta (function)
macro_rules! Depcrate_repository_configwrite_meta {
() => {
// Module: crate::repository::config
// Provides: {"write_meta"}
// Dependencies: {}
fn write_meta (meta : & gix :: config :: file :: Metadata , out : & mut impl std :: io :: Write) -> std :: io :: Result < () > { writeln ! (out , "# From '{}' ({:?}{}{})" , meta . path . as_deref () . map_or_else (|| "memory" . into () , | p | p . display () . to_string ()) , meta . source , if meta . level != 0 { format ! (", include level {}" , meta . level) } else { Default :: default () } , if meta . trust != gix :: sec :: Trust :: Full { ", untrusted" } else { Default :: default () }) }
};
}

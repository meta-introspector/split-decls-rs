// Generated macro for print_bsd_member_header (function)
macro_rules! Depcrate_archive_writerprint_bsd_member_header {
() => {
// Module: crate::archive_writer
// Provides: {"print_bsd_member_header"}
// Dependencies: {}
fn print_bsd_member_header < W : Write > (w : & mut W , pos : u64 , name : & str , mtime : u64 , uid : u32 , gid : u32 , perms : u32 , size : u64 ,) -> io :: Result < () > { let pos_after_header = pos + 60 + u64 :: try_from (name . len ()) . unwrap () ; let pad = offset_to_alignment (pos_after_header , 8) ; let name_with_padding = u64 :: try_from (name . len ()) . unwrap () + pad ; write ! (w , "#1/{name_with_padding:<13}") ? ; print_rest_of_member_header (w , mtime , uid , gid , perms , name_with_padding + size) ? ; write ! (w , "{name}") ? ; write ! (w , "{nil:\0<pad$}" , nil = "" , pad = usize :: try_from (pad) . unwrap ()) }
};
}

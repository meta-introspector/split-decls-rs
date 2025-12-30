// Generated macro for print_member_header (function)
macro_rules! Depcrate_archive_writerprint_member_header {
() => {
// Module: crate::archive_writer
// Provides: {"print_member_header"}
// Dependencies: {}
fn print_member_header < 'm , W : Write , T : Write + Seek > (w : & mut W , pos : u64 , string_table : & mut T , member_names : & mut HashMap < & 'm str , u64 > , kind : ArchiveKind , thin : bool , m : & 'm NewArchiveMember < 'm > , mtime : u64 , size : u64 ,) -> io :: Result < () > { if is_bsd_like (kind) { return print_bsd_member_header (w , pos , & m . member_name , mtime , m . uid , m . gid , m . perms , size) ; } if ! use_string_table (thin , & m . member_name) { return print_gnu_small_member_header (w , m . member_name . clone () , mtime , m . uid , m . gid , m . perms , size ,) ; } write ! (w , "/") ? ; let name_pos ; if thin { name_pos = string_table . stream_position () ? ; write ! (string_table , "{}/\n" , m . member_name) ? ; } else if let Some (& pos) = member_names . get (& * m . member_name) { name_pos = pos ; } else { name_pos = string_table . stream_position () ? ; member_names . insert (& m . member_name , name_pos) ; write ! (string_table , "{}" , m . member_name) ? ; if is_coff_archive (kind) { write ! (string_table , "\0") ? ; } else { write ! (string_table , "/\n") ? ; } } write ! (w , "{name_pos:<15}") ? ; print_rest_of_member_header (w , mtime , m . uid , m . gid , m . perms , size) }
};
}

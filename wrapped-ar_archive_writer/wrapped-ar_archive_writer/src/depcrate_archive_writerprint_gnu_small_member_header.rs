// Generated macro for print_gnu_small_member_header (function)
macro_rules! Depcrate_archive_writerprint_gnu_small_member_header {
() => {
// Module: crate::archive_writer
// Provides: {"print_gnu_small_member_header"}
// Dependencies: {}
fn print_gnu_small_member_header < W : Write > (w : & mut W , name : String , mtime : u64 , uid : u32 , gid : u32 , perms : u32 , size : u64 ,) -> io :: Result < () > { write ! (w , "{:<16}" , name + "/") ? ; print_rest_of_member_header (w , mtime , uid , gid , perms , size) }
};
}

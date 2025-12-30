// Generated macro for print_rest_of_member_header (function)
macro_rules! Depcrate_archive_writerprint_rest_of_member_header {
() => {
// Module: crate::archive_writer
// Provides: {"print_rest_of_member_header"}
// Dependencies: {}
fn print_rest_of_member_header < W : Write > (w : & mut W , mtime : u64 , uid : u32 , gid : u32 , perms : u32 , size : u64 ,) -> io :: Result < () > { write ! (w , "{:<12}{:<6}{:<6}{:<8o}{:<10}`\n" , mtime , uid % 1000000 , gid % 1000000 , perms , size) }
};
}

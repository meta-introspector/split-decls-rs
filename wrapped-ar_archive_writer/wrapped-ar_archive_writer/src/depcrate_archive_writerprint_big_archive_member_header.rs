// Generated macro for print_big_archive_member_header (function)
macro_rules! Depcrate_archive_writerprint_big_archive_member_header {
() => {
// Module: crate::archive_writer
// Provides: {"print_big_archive_member_header"}
// Dependencies: {}
fn print_big_archive_member_header < W : Write > (w : & mut W , name : & str , mtime : u64 , uid : u32 , gid : u32 , perms : u32 , size : u64 , prev_offset : u64 , next_offset : u64 ,) -> io :: Result < () > { write ! (w , "{:<20}{:<20}{:<20}{:<12}{:<12}{:<12}{:<12o}{:<4}" , size , next_offset , prev_offset , mtime , u64 :: from (uid) % 1000000000000u64 , u64 :: from (gid) % 1000000000000u64 , perms , name . len () ,) ? ; if ! name . is_empty () { write ! (w , "{name}") ? ; if name . len () % 2 != 0 { write ! (w , "\0") ? ; } } write ! (w , "`\n") ? ; Ok (()) }
};
}

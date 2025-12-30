// Generated macro for sys_getdents64 (function)
macro_rules! Depcrate_syscallssys_getdents64 {
() => {
// Module: crate::syscalls
// Provides: {"sys_getdents64"}
// Dependencies: {}
# [doc = " Read the entries of a directory."] # [doc = " Similar as the Linux system-call, this reads up to `count` bytes and returns the number of"] # [doc = " bytes written. If the size was not sufficient to list all directory entries, subsequent calls"] # [doc = " to this fn return the next entries."] # [doc = ""] # [doc = " Parameters:"] # [doc = ""] # [doc = " - `fd`: File Descriptor of the directory in question."] # [doc = " -`dirp`: Memory for the kernel to store the filled `Dirent64` objects including the c-strings with the filenames to."] # [doc = " - `count`: Size of the memory region described by `dirp` in bytes."] # [doc = ""] # [doc = " Return:"] # [doc = ""] # [doc = " The number of bytes read into `dirp` on success. Zero indicates that no more entries remain and"] # [doc = " the directories readposition needs to be reset using `sys_lseek`."] # [doc = " Negative numbers encode errors."] # [hermit_macro :: system (errno)] # [unsafe (no_mangle)] pub unsafe extern "C" fn sys_getdents64 (fd : FileDescriptor , dirp : * mut Dirent64 , count : usize ,) -> i64 { debug ! ("getdents for fd {fd:?} - count: {count}") ; if dirp . is_null () || count == 0 { return (- i32 :: from (Errno :: Inval)) . into () ; } let slice = unsafe { core :: slice :: from_raw_parts_mut (dirp . cast () , count) } ; let obj = get_object (fd) ; obj . map_or_else (| _ | (- i32 :: from (Errno :: Inval)) . into () , | v | { block_on (async { v . read () . await . getdents (slice) . await } , None) . map_or_else (| e | (- i32 :: from (e)) . into () , | cnt | cnt as i64) } ,) }
};
}

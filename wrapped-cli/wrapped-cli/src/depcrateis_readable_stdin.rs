// Generated macro for is_readable_stdin (function)
macro_rules! Depcrateis_readable_stdin {
() => {
// Module: crate
// Provides: {"is_readable_stdin"}
// Dependencies: {}
# [doc = " Returns true if and only if stdin is believed to be readable."] # [doc = ""] # [doc = " When stdin is readable, command line programs may choose to behave"] # [doc = " differently than when stdin is not readable. For example, `command foo`"] # [doc = " might search the current directory for occurrences of `foo` where as"] # [doc = " `command foo < some-file` or `cat some-file | command foo` might instead"] # [doc = " only search stdin for occurrences of `foo`."] # [doc = ""] # [doc = " Note that this isn't perfect and essentially corresponds to a heuristic."] # [doc = " When things are unclear (such as if an error occurs during introspection to"] # [doc = " determine whether stdin is readable), this prefers to return `false`. That"] # [doc = " means it's possible for an end user to pipe something into your program and"] # [doc = " have this return `false` and thus potentially lead to ignoring the user's"] # [doc = " stdin data. While not ideal, this is perhaps better than falsely assuming"] # [doc = " stdin is readable, which would result in blocking forever on reading stdin."] # [doc = " Regardless, commands should always provide explicit fallbacks to override"] # [doc = " behavior. For example, `rg foo -` will explicitly search stdin and `rg foo"] # [doc = " ./` will explicitly search the current working directory."] pub fn is_readable_stdin () -> bool { use std :: io :: IsTerminal ; # [cfg (unix)] fn imp () -> bool { use std :: { fs :: File , os :: { fd :: AsFd , unix :: fs :: FileTypeExt } , } ; let stdin = std :: io :: stdin () ; let fd = match stdin . as_fd () . try_clone_to_owned () { Ok (fd) => fd , Err (err) => { log :: debug ! ("for heuristic stdin detection on Unix, \
                     could not clone stdin file descriptor \
                     (thus assuming stdin is not readable): {err}" ,) ; return false ; } } ; let file = File :: from (fd) ; let md = match file . metadata () { Ok (md) => md , Err (err) => { log :: debug ! ("for heuristic stdin detection on Unix, \
                     could not get file metadata for stdin \
                     (thus assuming stdin is not readable): {err}" ,) ; return false ; } } ; let ft = md . file_type () ; let is_file = ft . is_file () ; let is_fifo = ft . is_fifo () ; let is_socket = ft . is_socket () ; let is_readable = is_file || is_fifo || is_socket ; log :: debug ! ("for heuristic stdin detection on Unix, \
             found that \
             is_file={is_file}, is_fifo={is_fifo} and is_socket={is_socket}, \
             and thus concluded that is_stdin_readable={is_readable}" ,) ; is_readable } # [cfg (windows)] fn imp () -> bool { let stdin = winapi_util :: HandleRef :: stdin () ; let typ = match winapi_util :: file :: typ (stdin) { Ok (typ) => typ , Err (err) => { log :: debug ! ("for heuristic stdin detection on Windows, \
                     could not get file type of stdin \
                     (thus assuming stdin is not readable): {err}" ,) ; return false ; } } ; let is_disk = typ . is_disk () ; let is_pipe = typ . is_pipe () ; let is_readable = is_disk || is_pipe ; log :: debug ! ("for heuristic stdin detection on Windows, \
             found that is_disk={is_disk} and is_pipe={is_pipe}, \
             and thus concluded that is_stdin_readable={is_readable}" ,) ; is_readable } # [cfg (not (any (unix , windows)))] fn imp () -> bool { log :: debug ! ("on non-{{Unix,Windows}}, assuming stdin is not readable") ; false } ! std :: io :: stdin () . is_terminal () && imp () }
};
}

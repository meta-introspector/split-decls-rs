// Generated macro for find_jobserver_auth (function)
macro_rules! Depcratefind_jobserver_auth {
() => {
// Module: crate
// Provides: {"find_jobserver_auth"}
// Dependencies: {}
# [doc = " Finds and returns the value of `--jobserver-auth=<VALUE>` in the given"] # [doc = " environment variable."] # [doc = ""] # [doc = " Precedence rules:"] # [doc = ""] # [doc = " * The last instance wins [^1]."] # [doc = " * `--jobserver-fds=` as a fallback when no `--jobserver-auth=` is present [^2]."] # [doc = ""] # [doc = " [^1]: See [\"GNU `make` manual: Sharing Job Slots with GNU `make`\"](https://www.gnu.org/software/make/manual/make.html#Job-Slots)"] # [doc = " _\"Be aware that the `MAKEFLAGS` variable may contain multiple instances of"] # [doc = " the `--jobserver-auth=` option. Only the last instance is relevant.\"_"] # [doc = ""] # [doc = " [^2]: Refer to [the release announcement](https://git.savannah.gnu.org/cgit/make.git/tree/NEWS?h=4.2#n31)"] # [doc = " of GNU Make 4.2, which states that `--jobserver-fds` was initially an"] # [doc = " internal-only flag and was later renamed to `--jobserver-auth`."] fn find_jobserver_auth (var : & str) -> Option < & str > { ["--jobserver-auth=" , "--jobserver-fds="] . iter () . find_map (| & arg | var . rsplit_once (arg) . map (| (_ , s) | s)) . and_then (| s | s . split (' ') . next ()) }
};
}

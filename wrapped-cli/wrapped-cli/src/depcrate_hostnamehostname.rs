// Generated macro for hostname (function)
macro_rules! Depcrate_hostnamehostname {
() => {
// Module: crate::hostname
// Provides: {"hostname"}
// Dependencies: {}
# [doc = " Returns the hostname of the current system."] # [doc = ""] # [doc = " It is unusual, although technically possible, for this routine to return"] # [doc = " an error. It is difficult to list out the error conditions, but one such"] # [doc = " possibility is platform support."] # [doc = ""] # [doc = " # Platform specific behavior"] # [doc = ""] # [doc = " On Windows, this currently uses the \"physical DNS hostname\" computer name."] # [doc = " This may change in the future."] # [doc = ""] # [doc = " On Unix, this returns the result of the `gethostname` function from the"] # [doc = " `libc` linked into the program."] pub fn hostname () -> io :: Result < OsString > { # [cfg (windows)] { use winapi_util :: sysinfo :: { ComputerNameKind , get_computer_name } ; get_computer_name (ComputerNameKind :: PhysicalDnsHostname) } # [cfg (unix)] { gethostname () } # [cfg (not (any (windows , unix)))] { Err (io :: Error :: new (io :: ErrorKind :: Other , "hostname could not be found on unsupported platform" ,)) } }
};
}

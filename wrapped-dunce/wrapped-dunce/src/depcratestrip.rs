// Generated macro for strip (function)
macro_rules! Depcratestrip {
() => {
// Module: crate
// Provides: {"strip"}
// Dependencies: {}
# [test] # [cfg (windows)] fn strip () { assert_eq ! (Path :: new (r"C:\foo\😀") , simplified (Path :: new (r"\\?\C:\foo\😀"))) ; assert_eq ! (Path :: new (r"\\?\serv\") , simplified (Path :: new (r"\\?\serv\"))) ; assert_eq ! (Path :: new (r"\\.\C:\notdisk") , simplified (Path :: new (r"\\.\C:\notdisk"))) ; assert_eq ! (Path :: new (r"\\?\GLOBALROOT\Device\ImDisk0\path\to\file.txt") , simplified (Path :: new (r"\\?\GLOBALROOT\Device\ImDisk0\path\to\file.txt"))) ; }
};
}

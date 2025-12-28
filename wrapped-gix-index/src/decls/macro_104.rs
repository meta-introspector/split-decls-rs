macro_rules! deps {
    () => {
        Mode!();
    };
}

macro_rules! macro_104 {
    () => {
        deps!();
        bitflags ! { # [doc = " The kind of file of an entry."] # [derive (Copy , Clone , Debug , PartialEq , Eq , Ord , PartialOrd)] pub struct Mode : u32 { # [doc = " directory (only used for sparse checkouts), equivalent to a tree, which is _excluded_ from the index via"] # [doc = " cone-mode."] const DIR = 0o040000 ; # [doc = " regular file"] const FILE = 0o100644 ; # [doc = " regular file, executable"] const FILE_EXECUTABLE = 0o100755 ; # [doc = " Symbolic link"] const SYMLINK = 0o120000 ; # [doc = " A git commit for submodules"] const COMMIT = 0o160000 ; } }
    };
}

macro_104!();
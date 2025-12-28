macro_rules! PackageRoot {
    () => {
        # [doc = " `PackageRoot` describes a package root folder."] # [doc = " Which may be an external dependency, or a member of"] # [doc = " the current workspace."] # [derive (Debug , Clone , Eq , PartialEq , Hash)] pub struct PackageRoot { # [doc = " Is from the local filesystem and may be edited"] pub is_local : bool , # [doc = " Directories to include"] pub include : Vec < AbsPathBuf > , # [doc = " Directories to exclude"] pub exclude : Vec < AbsPathBuf > , }
    };
}

PackageRoot!()
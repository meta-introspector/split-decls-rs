macro_rules! ErrorKind {
    () => {
        # [doc = " A list specifying general categories of fs_extra error."] # [derive (Debug)] pub enum ErrorKind { # [doc = " An entity was not found."] NotFound , # [doc = " The operation lacked the necessary privileges to complete."] PermissionDenied , # [doc = " An entity already exists."] AlreadyExists , # [doc = " This operation was interrupted."] Interrupted , # [doc = " Path does not a directory."] InvalidFolder , # [doc = " Path does not a file."] InvalidFile , # [doc = " Invalid file name."] InvalidFileName , # [doc = " Invalid path."] InvalidPath , # [doc = " Any I/O error."] Io (IoError) , # [doc = " Any StripPrefix error."] StripPrefix (StripPrefixError) , # [doc = " Any OsString error."] OsString (OsString) , # [doc = " Any fs_extra error not part of this list."] Other , }
    };
}

ErrorKind!();
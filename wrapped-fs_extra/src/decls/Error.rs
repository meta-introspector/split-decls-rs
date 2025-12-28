macro_rules! deps {
    () => {
        ErrorKind!();
    };
}

macro_rules! Error {
    () => {
        deps!();
        # [doc = " The error type for fs_extra operations with files and folder."] # [doc = ""] # [doc = " Errors mostly originate from the underlying OS, but custom instances of"] # [doc = " `Error` can be created with crafted error messages and a particular value of"] # [doc = " [`ErrorKind`]."] # [doc = ""] # [doc = " [`ErrorKind`]: enum.ErrorKind.html"] # [derive (Debug)] pub struct Error { # [doc = " Type error"] pub kind : ErrorKind , message : String , }
    };
}

Error!()
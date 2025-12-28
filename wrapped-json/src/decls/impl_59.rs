macro_rules! deps {
    () => {
        ErrorCode!();
        Category!();
        Error!();
    };
}

macro_rules! impl_59 {
    () => {
        deps!();
        # [cfg (feature = "std")] # [allow (clippy :: fallible_impl_from)] impl From < Error > for io :: Error { # [doc = " Convert a `serde_json::Error` into an `io::Error`."] # [doc = ""] # [doc = " JSON syntax and data errors are turned into `InvalidData` I/O errors."] # [doc = " EOF errors are turned into `UnexpectedEof` I/O errors."] # [doc = ""] # [doc = " ```"] # [doc = " use std::io;"] # [doc = ""] # [doc = " enum MyError {"] # [doc = "     Io(io::Error),"] # [doc = "     Json(serde_json::Error),"] # [doc = " }"] # [doc = ""] # [doc = " impl From<serde_json::Error> for MyError {"] # [doc = "     fn from(err: serde_json::Error) -> MyError {"] # [doc = "         use serde_json::error::Category;"] # [doc = "         match err.classify() {"] # [doc = "             Category::Io => {"] # [doc = "                 MyError::Io(err.into())"] # [doc = "             }"] # [doc = "             Category::Syntax | Category::Data | Category::Eof => {"] # [doc = "                 MyError::Json(err)"] # [doc = "             }"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] fn from (j : Error) -> Self { if let ErrorCode :: Io (err) = j . err . code { err } else { match j . classify () { Category :: Io => unreachable ! () , Category :: Syntax | Category :: Data => io :: Error :: new (ErrorKind :: InvalidData , j) , Category :: Eof => io :: Error :: new (ErrorKind :: UnexpectedEof , j) , } } } }
    };
}

impl_59!();
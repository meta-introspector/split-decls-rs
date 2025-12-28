macro_rules! deps {
    () => {
        ErrorKind!();
        Error!();
    };
}

macro_rules! impl_10 {
    () => {
        deps!();
        impl From < IoError > for Error { fn from (err : IoError) -> Error { let err_kind : ErrorKind ; match err . kind () { IoErrorKind :: NotFound => err_kind = ErrorKind :: NotFound , IoErrorKind :: PermissionDenied => err_kind = ErrorKind :: PermissionDenied , IoErrorKind :: AlreadyExists => err_kind = ErrorKind :: AlreadyExists , IoErrorKind :: Interrupted => err_kind = ErrorKind :: Interrupted , IoErrorKind :: Other => err_kind = ErrorKind :: Other , _ => { err_kind = ErrorKind :: Io (err) ; return Error :: new (err_kind , "Io error. Look inside err_kind for more details.") ; } } Error :: new (err_kind , & err . to_string ()) } }
    };
}

impl_10!()
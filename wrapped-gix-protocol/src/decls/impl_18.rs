macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl From < std :: io :: Error > for Error { fn from (err : std :: io :: Error) -> Self { if err . kind () == std :: io :: ErrorKind :: Other { match err . into_inner () { Some (err) => match err . downcast :: < gix_transport :: packetline :: read :: Error > () { Ok (err) => Error :: UploadPack (* err) , Err (err) => Error :: Io (std :: io :: Error :: other (err)) , } , None => Error :: Io (std :: io :: ErrorKind :: Other . into ()) , } } else { Error :: Io (err) } } }
    };
}

impl_18!()
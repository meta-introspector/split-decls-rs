macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! impl_55 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < Error > for std :: io :: Error { fn from (err : Error) -> std :: io :: Error { std :: io :: Error :: new (std :: io :: ErrorKind :: InvalidData , err) } }
    };
}

impl_55!()
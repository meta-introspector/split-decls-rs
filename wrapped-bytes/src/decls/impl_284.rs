macro_rules! deps {
    () => {
        TryGetError!();
    };
}

macro_rules! impl_284 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl From < TryGetError > for std :: io :: Error { fn from (error : TryGetError) -> Self { std :: io :: Error :: new (std :: io :: ErrorKind :: Other , error) } }
    };
}

impl_284!()
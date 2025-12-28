macro_rules! deps {
    () => {
        Adapter!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < W > Adapter < W > where W : FnMut (& [u8]) -> std :: io :: Result < () > , { pub (crate) fn new (writer : W) -> Self { Adapter { writer , error : Ok (()) , } } pub (crate) fn write_fmt (mut self , fmt : std :: fmt :: Arguments < '_ >) -> std :: io :: Result < () > { match std :: fmt :: write (& mut self , fmt) { Ok (()) => Ok (()) , Err (..) => { if self . error . is_err () { self . error } else { Err (std :: io :: Error :: new (std :: io :: ErrorKind :: Other , "formatter error" ,)) } } } } }
    };
}

impl_109!();
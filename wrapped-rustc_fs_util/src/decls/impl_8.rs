macro_rules! deps {
    () => {
        TempDirBuilder!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < 'a , 'b > TempDirBuilder < 'a , 'b > { pub fn new () -> Self { Self { builder : tempfile :: Builder :: new () } } pub fn prefix < S : AsRef < OsStr > + ? Sized > (& mut self , prefix : & 'a S) -> & mut Self { self . builder . prefix (prefix) ; self } pub fn suffix < S : AsRef < OsStr > + ? Sized > (& mut self , suffix : & 'b S) -> & mut Self { self . builder . suffix (suffix) ; self } pub fn tempdir_in < P : AsRef < Path > > (& self , dir : P) -> io :: Result < TempDir > { let dir = dir . as_ref () ; # [cfg (windows)] for wait in 1 .. 11 { match self . builder . tempdir_in (dir) { Err (e) if e . kind () == io :: ErrorKind :: PermissionDenied => { } t => return t , } std :: thread :: sleep (std :: time :: Duration :: from_millis (1 << wait)) ; } self . builder . tempdir_in (dir) } pub fn tempdir (& self) -> io :: Result < TempDir > { self . tempdir_in (env :: temp_dir ()) } }
    };
}

impl_8!()
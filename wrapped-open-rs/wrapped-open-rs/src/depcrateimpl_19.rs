// Generated macro for impl_19 (impl)
macro_rules! Depcrateimpl_19 {
() => {
// Module: crate
// Provides: {"impl_19"}
// Dependencies: {}
impl IntoResult < io :: Result < () > > for io :: Result < std :: process :: ExitStatus > { fn into_result (self , cmd : & Command) -> io :: Result < () > { match self { Ok (status) if status . success () => Ok (()) , Ok (status) => Err (io :: Error :: new (io :: ErrorKind :: Other , format ! ("Launcher {cmd:?} failed with {:?}" , status) ,)) , Err (err) => Err (err) , } } }
};
}

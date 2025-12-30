// Generated macro for impl_95 (impl)
macro_rules! Depcrate_tempfileimpl_95 {
() => {
// Module: crate::tempfile
// Provides: {"impl_95"}
// Dependencies: {}
impl NamedTempfile { pub (super) fn new (base : & Path , suffix : & str) -> io :: Result < Self > { for _ in 0 .. 10 { let path = base . join (tmpname (suffix)) ; match create_named (& path) { Ok (file) => { return Ok (Self { file : Some (file) , path , }) } Err (e) if e . kind () == io :: ErrorKind :: AlreadyExists => continue , Err (e) => return Err (e) , } ; } Err (io :: Error :: new (io :: ErrorKind :: AlreadyExists , format ! ("too many temporary files exist in base `{}` with suffix `{}`" , base . display () , suffix) ,)) } pub (super) fn path (& self) -> & Path { & self . path } pub (super) fn take_file (& mut self) -> Option < File > { self . file . take () } }
};
}

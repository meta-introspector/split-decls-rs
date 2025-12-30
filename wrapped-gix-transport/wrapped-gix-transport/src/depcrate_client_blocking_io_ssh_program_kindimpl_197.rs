// Generated macro for impl_197 (impl)
macro_rules! Depcrate_client_blocking_io_ssh_program_kindimpl_197 {
() => {
// Module: crate::client::blocking_io::ssh::program_kind
// Provides: {"impl_197"}
// Dependencies: {}
impl < 'a > From < & 'a OsStr > for ProgramKind { fn from (v : & 'a OsStr) -> Self { let p = std :: path :: Path :: new (v) ; match p . file_stem () . and_then (OsStr :: to_str) { None => ProgramKind :: Simple , Some (stem) => { if stem . eq_ignore_ascii_case ("ssh") { ProgramKind :: Ssh } else if stem . eq_ignore_ascii_case ("plink") { ProgramKind :: Plink } else if stem . eq_ignore_ascii_case ("putty") { ProgramKind :: Putty } else if stem . eq_ignore_ascii_case ("tortoiseplink") { ProgramKind :: TortoisePlink } else { ProgramKind :: Simple } } } } }
};
}

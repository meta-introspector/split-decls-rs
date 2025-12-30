// Generated macro for verify_uefi_rlib_format (function)
macro_rules! Depcrate_core_build_steps_distverify_uefi_rlib_format {
() => {
// Module: crate::core::build_steps::dist
// Provides: {"verify_uefi_rlib_format"}
// Dependencies: {}
# [doc = " Check that all objects in rlibs for UEFI targets are COFF. This"] # [doc = " ensures that the C compiler isn't producing ELF objects, which would"] # [doc = " not link correctly with the COFF objects."] fn verify_uefi_rlib_format (builder : & Builder < '_ > , target : TargetSelection , stamp : & BuildStamp) { if ! target . ends_with ("-uefi") { return ; } for (path , _) in builder . read_stamp_file (stamp) { if path . extension () != Some (OsStr :: new ("rlib")) { continue ; } let data = t ! (fs :: read (& path)) ; let data = data . as_slice () ; let archive = t ! (ArchiveFile :: parse (data)) ; for member in archive . members () { let member = t ! (member) ; let member_data = t ! (member . data (data)) ; let is_coff = match object :: File :: parse (member_data) { Ok (member_file) => member_file . format () == BinaryFormat :: Coff , Err (_) => false , } ; if ! is_coff { let member_name = String :: from_utf8_lossy (member . name ()) ; panic ! ("member {} in {} is not COFF" , member_name , path . display ()) ; } } } }
};
}

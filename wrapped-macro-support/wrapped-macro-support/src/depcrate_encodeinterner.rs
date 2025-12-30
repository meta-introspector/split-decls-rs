// Generated macro for Interner (struct)
macro_rules! Depcrate_encodeInterner {
() => {
// Module: crate::encode
// Provides: {"Interner"}
// Dependencies: {}
struct Interner { bump : bumpalo :: Bump , files : RefCell < HashMap < String , LocalFile > > , root : PathBuf , crate_name : String , has_package_json : Cell < bool > , }
};
}

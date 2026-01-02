// Custom macro to include processed rustc files
macro_rules! include_rustc {
    ($crate_name:ident, $file:ident) => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/submodules/rust/compiler/", stringify!($crate_name), "/src/", stringify!($file), ".rs"));
    };
    ($crate_name:ident) => {
        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/submodules/rust/compiler/", stringify!($crate_name), "/src/lib.rs"));
    };
}

// Include processed rustc files using the macro
include_rustc!(rustc_session);
include_rustc!(rustc_session, session);
include_rustc!(rustc_session, parse);
include_rustc!(rustc_session, config);
include_rustc!(rustc_errors);

// Only keep types that don't exist in processed files
pub struct FileName;

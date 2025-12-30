// Generated macro for save (function)
macro_rules! Depcrate_fssave {
() => {
// Module: crate::fs
// Provides: {"save"}
// Dependencies: {}
pub fn save < D , P > (data : & D , path : & P) -> Result < () > where D : Serialize , P : AsRef < Path > , { let buf = serde_json :: to_string (& data) . map_err (| inner | Error :: SerdeError { path : path . as_ref () . to_owned () , inner , }) ? ; save_string (& buf , path) }
};
}

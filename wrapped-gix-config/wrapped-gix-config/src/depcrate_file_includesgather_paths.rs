// Generated macro for gather_paths (function)
macro_rules! Depcrate_file_includesgather_paths {
() => {
// Module: crate::file::includes
// Provides: {"gather_paths"}
// Dependencies: {}
fn gather_paths (section : & file :: Section < '_ > , id : SectionId) -> Vec < (SectionId , crate :: Path < 'static >) > { section . body . values ("path") . into_iter () . map (| path | (id , crate :: Path :: from (Cow :: Owned (path . into_owned ())))) . collect () }
};
}

// Generated macro for local_crates (function)
macro_rules! Depcrate_utillocal_crates {
() => {
// Module: crate::util
// Provides: {"local_crates"}
// Dependencies: {}
# [doc = " Pulls all the crates in this workspace from the cargo metadata."] # [doc = " Additionally, somewhere between cargo metadata and TyCtxt, '-' gets replaced with '_' so we"] # [doc = " make that same transformation here."] pub fn local_crates (metadata : & Metadata) -> String { assert ! (! metadata . workspace_members . is_empty ()) ; let package_name_by_id : HashMap < _ , _ > = metadata . packages . iter () . map (| package | (& package . id , package . name . as_str ())) . collect () ; metadata . workspace_members . iter () . map (| id | package_name_by_id [id] . replace ('-' , "_")) . collect :: < Vec < _ > > () . join (",") }
};
}

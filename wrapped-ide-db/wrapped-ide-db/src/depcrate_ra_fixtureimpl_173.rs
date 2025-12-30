// Generated macro for impl_173 (impl)
macro_rules! Depcrate_ra_fixtureimpl_173 {
() => {
// Module: crate::ra_fixture
// Provides: {"impl_173"}
// Dependencies: {}
impl RootDatabase { fn from_ra_fixture (text : & str , minicore : MiniCore < '_ > ,) -> Result < (RootDatabase , Vec < (FileId , usize) > , Vec < FileId >) , () > { std :: panic :: catch_unwind (| | { let mut db = RootDatabase :: default () ; let fixture = test_fixture :: ChangeFixture :: parse_with_proc_macros (& db , text , minicore . 0 , Vec :: new () ,) ; db . apply_change (fixture . change) ; let files = fixture . files . into_iter () . zip (fixture . file_lines) . map (| (file_id , range) | (file_id . file_id (& db) , range)) . collect () ; (db , files , fixture . sysroot_files) }) . map_err (| error | { tracing :: error ! ("cannot crate the crate graph: {}\nCrate graph:\n{}\n" , if let Some (& s) = error . downcast_ref ::<&'static str > () { s } else if let Some (s) = error . downcast_ref ::< String > () { s . as_str () } else { "Box<dyn Any>" } , text ,) ; }) } }
};
}

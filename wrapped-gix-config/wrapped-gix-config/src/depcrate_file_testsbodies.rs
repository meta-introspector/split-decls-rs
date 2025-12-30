// Generated macro for bodies (function)
macro_rules! Depcrate_file_testsbodies {
() => {
// Module: crate::file::tests
// Provides: {"bodies"}
// Dependencies: {}
fn bodies < 'a > (sections : & HashMap < SectionId , Section < 'a > >) -> HashMap < SectionId , file :: section :: Body < 'a > > { sections . iter () . map (| (k , v) | (* k , v . body . clone ())) . collect () }
};
}

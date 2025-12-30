// Generated macro for headers (function)
macro_rules! Depcrate_file_testsheaders {
() => {
// Module: crate::file::tests
// Provides: {"headers"}
// Dependencies: {}
fn headers < 'a > (sections : & HashMap < SectionId , Section < 'a > >) -> HashMap < SectionId , section :: Header < 'a > > { sections . iter () . map (| (k , v) | (* k , v . header . clone ())) . collect () }
};
}

// Generated macro for safe (function)
macro_rules! Depcratesafe {
() => {
// Module: crate
// Provides: {"safe"}
// Dependencies: {}
# [test] # [cfg (windows)] fn safe () { assert ! (try_simplified (Path :: new (r"\\?\C:\foo\bar")) . is_some ()) ; assert ! (try_simplified (Path :: new (r"\\?\Z:\foo\bar\")) . is_some ()) ; assert ! (try_simplified (Path :: new (r"\\?\Z:\😀\🎃\")) . is_some ()) ; assert ! (try_simplified (Path :: new (r"\\?\c:\foo")) . is_some ()) ; let long = :: std :: iter :: repeat ("®") . take (160) . collect :: < String > () ; assert ! (try_simplified (Path :: new (& format ! (r"\\?\c:\{}" , long))) . is_some ()) ; assert ! (! try_simplified (Path :: new (& format ! (r"\\?\c:\{}\{}" , long , long))) . is_some ()) ; assert ! (! try_simplified (Path :: new (r"\\?\C:\foo\.\bar")) . is_some ()) ; assert ! (! try_simplified (Path :: new (r"\\?\C:\foo\..\bar")) . is_some ()) ; assert ! (! try_simplified (Path :: new (r"\\?\c\foo")) . is_some ()) ; assert ! (! try_simplified (Path :: new (r"\\?\c\foo/bar")) . is_some ()) ; assert ! (! try_simplified (Path :: new (r"\\?\c:foo")) . is_some ()) ; assert ! (! try_simplified (Path :: new (r"\\?\cc:foo")) . is_some ()) ; assert ! (! try_simplified (Path :: new (r"\\?\c:foo\bar")) . is_some ()) ; }
};
}

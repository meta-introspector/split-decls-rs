macro_rules! minimize_rpaths {
    () => {
        fn minimize_rpaths (rpaths : & [OsString]) -> Vec < OsString > { let mut set = FxHashSet :: default () ; let mut minimized = Vec :: new () ; for rpath in rpaths { if set . insert (rpath) { minimized . push (rpath . clone ()) ; } } minimized }
    };
}

minimize_rpaths!()
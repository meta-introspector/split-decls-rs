macro_rules! deps {
    () => {
        TestCase!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl TestCase { fn list (path : & 'static str) -> Vec < TestCase > { let crate_root_dir = Path :: new (env ! ("CARGO_MANIFEST_DIR")) ; let test_data_dir = crate_root_dir . join ("test_data") ; let dir = test_data_dir . join (path) ; let mut res = Vec :: new () ; let read_dir = fs :: read_dir (& dir) . unwrap_or_else (| err | panic ! ("can't `read_dir` {}: {err}" , dir . display ())) ; for file in read_dir { let file = file . unwrap () ; let path = file . path () ; if path . extension () . unwrap_or_default () == "rs" { let rs = path ; let rast = rs . with_extension ("rast") ; let text = fs :: read_to_string (& rs) . unwrap () ; res . push (TestCase { rs , rast , text }) ; } } res . sort () ; res } }
    };
}

impl_106!()
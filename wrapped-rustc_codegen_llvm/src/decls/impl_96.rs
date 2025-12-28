macro_rules! deps {
    () => {
        ThinLTOModule!();
        ThinLTOKeysMap!();
        ThinData!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl ThinLTOKeysMap { fn save_to_file (& self , path : & Path) -> io :: Result < () > { use std :: io :: Write ; let mut writer = File :: create_buffered (path) ? ; for (module , key) in & self . keys { writeln ! (writer , "{module} {key}") ? ; } Ok (()) } fn load_from_file (path : & Path) -> io :: Result < Self > { use std :: io :: BufRead ; let mut keys = BTreeMap :: default () ; let file = File :: open_buffered (path) ? ; for line in file . lines () { let line = line ? ; let mut split = line . split (' ') ; let module = split . next () . unwrap () ; let key = split . next () . unwrap () ; assert_eq ! (split . next () , None , "Expected two space-separated values, found {line:?}") ; keys . insert (module . to_string () , key . to_string ()) ; } Ok (Self { keys }) } fn from_thin_lto_modules (data : & ThinData , modules : & [llvm :: ThinLTOModule] , names : & [CString] ,) -> Self { let keys = iter :: zip (modules , names) . map (| (module , name) | { let key = build_string (| rust_str | unsafe { llvm :: LLVMRustComputeLTOCacheKey (rust_str , module . identifier , data . 0) ; }) . expect ("Invalid ThinLTO module key") ; (module_name_to_str (name) . to_string () , key) }) . collect () ; Self { keys } } }
    };
}

impl_96!();
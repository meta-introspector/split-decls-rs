macro_rules! deps {
    () => {
        Result!();
        VTabLog!();
        VTabConnection!();
        Error!();
    };
}

macro_rules! impl_645 {
    () => {
        deps!();
        impl VTabLog { fn connect_create (db : & mut VTabConnection , _ : Option < & () > , args : & [& [u8]] , is_create : bool ,) -> Result < (String , Self) > { static N_INST : AtomicUsize = AtomicUsize :: new (1) ; let i_inst = N_INST . fetch_add (1 , Ordering :: SeqCst) ; println ! ("VTabLog::{}(tab={}, args={:?}):" , if is_create { "create" } else { "connect" } , i_inst , args . iter () . map (| b | str :: from_utf8 (b)) . collect ::< Vec < _ >> () ,) ; let mut schema = None ; let mut n_row = None ; let args = & args [3 ..] ; for c_slice in args { let (param , value) = super :: parameter (c_slice) ? ; match param { "schema" => { if schema . is_some () { return Err (Error :: ModuleError (format ! ("more than one '{param}' parameter"))) ; } schema = Some (value . to_owned ()) } "rows" => { if n_row . is_some () { return Err (Error :: ModuleError (format ! ("more than one '{param}' parameter"))) ; } if let Ok (n) = i64 :: from_str (value) { n_row = Some (n) } } _ => { return Err (Error :: ModuleError (format ! ("unrecognized parameter '{param}'"))) ; } } } if schema . is_none () { return Err (Error :: ModuleError ("no schema defined" . to_owned ())) ; } let vtab = Self { base : ffi :: sqlite3_vtab :: default () , db : unsafe { db . handle () } , n_row : n_row . unwrap_or (10) , i_inst , n_cursor : 0 , } ; Ok ((schema . unwrap () , vtab)) } }
    };
}

impl_645!()
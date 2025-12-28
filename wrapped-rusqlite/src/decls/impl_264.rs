macro_rules! deps {
    () => {
        Changeset!();
        Connection!();
        Result!();
        ConflictAction!();
        ChangesetItem!();
        ConflictType!();
    };
}

macro_rules! impl_264 {
    () => {
        deps!();
        impl Connection { # [doc = " Apply a changeset to a database"] pub fn apply < F , C > (& self , cs : & Changeset , filter : Option < F > , conflict : C) -> Result < () > where F : Fn (& str) -> bool + Send + 'static , C : Fn (ConflictType , ChangesetItem) -> ConflictAction + Send + 'static , { let db = self . db . borrow_mut () . db ; let filtered = filter . is_some () ; let tuple = & mut (filter , conflict) ; check (unsafe { if filtered { ffi :: sqlite3changeset_apply (db , cs . n , cs . cs , Some (call_filter :: < F , C >) , Some (call_conflict :: < F , C >) , tuple as * mut (Option < F > , C) as * mut c_void ,) } else { ffi :: sqlite3changeset_apply (db , cs . n , cs . cs , None , Some (call_conflict :: < F , C >) , tuple as * mut (Option < F > , C) as * mut c_void ,) } }) } # [doc = " Apply a changeset to a database"] pub fn apply_strm < F , C > (& self , input : & mut dyn Read , filter : Option < F > , conflict : C ,) -> Result < () > where F : Fn (& str) -> bool + Send + 'static , C : Fn (ConflictType , ChangesetItem) -> ConflictAction + Send + 'static , { let input_ref = & input ; let db = self . db . borrow_mut () . db ; let filtered = filter . is_some () ; let tuple = & mut (filter , conflict) ; check (unsafe { if filtered { ffi :: sqlite3changeset_apply_strm (db , Some (x_input) , input_ref as * const & mut dyn Read as * mut c_void , Some (call_filter :: < F , C >) , Some (call_conflict :: < F , C >) , tuple as * mut (Option < F > , C) as * mut c_void ,) } else { ffi :: sqlite3changeset_apply_strm (db , Some (x_input) , input_ref as * const & mut dyn Read as * mut c_void , None , Some (call_conflict :: < F , C >) , tuple as * mut (Option < F > , C) as * mut c_void ,) } }) } }
    };
}

impl_264!();
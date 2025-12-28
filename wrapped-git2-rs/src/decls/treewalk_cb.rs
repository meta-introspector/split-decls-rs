macro_rules! deps {
    () => {
        TreeWalkCbData!();
    };
}

macro_rules! treewalk_cb {
    () => {
        deps!();
        extern "C" fn treewalk_cb < T : Into < i32 > > (root : * const c_char , entry : * const raw :: git_tree_entry , payload : * mut c_void ,) -> c_int { match panic :: wrap (| | unsafe { let root = match CStr :: from_ptr (root) . to_str () { Ok (value) => value , _ => return - 1 , } ; let entry = entry_from_raw_const (entry) ; let payload = & mut * (payload as * mut TreeWalkCbData < '_ , T >) ; let callback = & mut payload . callback ; callback (root , & entry) . into () }) { Some (value) => value , None => - 1 , } }
    };
}

treewalk_cb!();
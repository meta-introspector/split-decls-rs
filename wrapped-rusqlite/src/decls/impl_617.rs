macro_rules! deps {
    () => {
        Result!();
        VTabCursor!();
        Context!();
        Filters!();
        ArrayTabCursor!();
    };
}

macro_rules! impl_617 {
    () => {
        deps!();
        unsafe impl VTabCursor for ArrayTabCursor < '_ > { fn filter (& mut self , idx_num : c_int , _idx_str : Option < & str > , args : & Filters < '_ >) -> Result < () > { if idx_num > 0 { self . ptr = args . get_array (0) ; } else { self . ptr = None ; } self . row_id = 1 ; Ok (()) } fn next (& mut self) -> Result < () > { self . row_id += 1 ; Ok (()) } fn eof (& self) -> bool { self . row_id > self . len () } fn column (& self , ctx : & mut Context , i : c_int) -> Result < () > { match i { CARRAY_COLUMN_POINTER => Ok (()) , _ => { if let Some (ref array) = self . ptr { let value = & array [(self . row_id - 1) as usize] ; ctx . set_result (& value) } else { Ok (()) } } } } fn rowid (& self) -> Result < i64 > { Ok (self . row_id) } }
    };
}

impl_617!();
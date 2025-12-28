macro_rules! deps {
    () => {
        Null!();
        Result!();
        Error!();
        Filters!();
        Context!();
        VTabCursor!();
        CsvTabCursor!();
    };
}

macro_rules! impl_627 {
    () => {
        deps!();
        unsafe impl VTabCursor for CsvTabCursor < '_ > { fn filter (& mut self , _idx_num : c_int , _idx_str : Option < & str > , _args : & Filters < '_ > ,) -> Result < () > { { let offset_first_row = self . vtab () . offset_first_row . clone () ; self . reader . seek (offset_first_row) ? ; } self . row_number = 0 ; self . next () } fn next (& mut self) -> Result < () > { { self . eof = self . reader . is_done () ; if self . eof { return Ok (()) ; } self . eof = ! self . reader . read_record (& mut self . cols) ? ; } self . row_number += 1 ; Ok (()) } fn eof (& self) -> bool { self . eof } fn column (& self , ctx : & mut Context , col : c_int) -> Result < () > { if col < 0 || col as usize >= self . cols . len () { return Err (Error :: ModuleError (format ! ("column index out of bounds: {col}"))) ; } if self . cols . is_empty () { return ctx . set_result (& Null) ; } ctx . set_result (& self . cols [col as usize] . to_owned ()) } fn rowid (& self) -> Result < i64 > { Ok (self . row_number as i64) } }
    };
}

impl_627!()
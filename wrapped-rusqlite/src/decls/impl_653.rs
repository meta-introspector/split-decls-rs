macro_rules! deps {
    () => {
        Context!();
        VTabLogCursor!();
        ValueRef!();
        Filters!();
        Result!();
        Type!();
        VTabCursor!();
        Null!();
    };
}

macro_rules! impl_653 {
    () => {
        deps!();
        unsafe impl VTabCursor for VTabLogCursor < '_ > { fn filter (& mut self , idx_num : c_int , idx_str : Option < & str > , args : & Filters < '_ >) -> Result < () > { println ! ("VTabLogCursor::filter(tab={}, cursor={}, idx_num={idx_num}, idx_str={idx_str:?}, args={})" , self . vtab () . i_inst , self . i_cursor , args . len ()) ; for (i , arg) in args . iter () . enumerate () { if arg . data_type () == Type :: Null { println ! (" in_values[{}]: {:?}" , i , args . in_values (i) ?. collect ::< Vec < ValueRef >> ()) ; } } self . row_id = 0 ; Ok (()) } fn next (& mut self) -> Result < () > { println ! ("VTabLogCursor::next(tab={}, cursor={}): rowid {} -> {}" , self . vtab () . i_inst , self . i_cursor , self . row_id , self . row_id + 1) ; self . row_id += 1 ; Ok (()) } fn eof (& self) -> bool { let eof = self . row_id >= self . vtab () . n_row ; println ! ("VTabLogCursor::eof(tab={}, cursor={}): {}" , self . vtab () . i_inst , self . i_cursor , eof ,) ; eof } fn column (& self , ctx : & mut Context , i : c_int) -> Result < () > { if ctx . no_change () { println ! ("VTabLogCursor::column(tab={}, cursor={}, i={}): no change" , self . vtab () . i_inst , self . i_cursor , i ,) ; return Ok (()) ; } let value = if i < 26 { format ! ("{}{}" , "abcdefghijklmnopqrstuvwyz" . chars () . nth (i as usize) . unwrap () , self . row_id) } else { format ! ("{i}{}" , self . row_id) } ; println ! ("VTabLogCursor::column(tab={}, cursor={}, i={}): {}" , self . vtab () . i_inst , self . i_cursor , i , value ,) ; if i == 0 { println ! ("  db bust: {:?}" , unsafe { ctx . get_connection () . map (| c | c . is_busy ()) }) } ctx . set_result (& value) } fn rowid (& self) -> Result < i64 > { println ! ("VTabLogCursor::rowid(tab={}, cursor={}): {}" , self . vtab () . i_inst , self . i_cursor , self . row_id ,) ; Ok (self . row_id) } }
    };
}

impl_653!()
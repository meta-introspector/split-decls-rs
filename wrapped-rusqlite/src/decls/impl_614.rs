macro_rules! deps {
    () => {
        ArrayTab!();
        IndexInfo!();
        IndexConstraintOp!();
        ArrayTabCursor!();
        VTab!();
        Result!();
        VTabConnection!();
    };
}

macro_rules! impl_614 {
    () => {
        deps!();
        unsafe impl < 'vtab > VTab < 'vtab > for ArrayTab { type Aux = () ; type Cursor = ArrayTabCursor < 'vtab > ; fn connect (_ : & mut VTabConnection , _aux : Option < & () > , _args : & [& [u8]] ,) -> Result < (String , Self) > { let vtab = Self { base : ffi :: sqlite3_vtab :: default () , } ; Ok (("CREATE TABLE x(value,pointer hidden)" . to_owned () , vtab)) } fn best_index (& self , info : & mut IndexInfo) -> Result < () > { let mut ptr_idx = false ; for (constraint , mut constraint_usage) in info . constraints_and_usages () { if ! constraint . is_usable () { continue ; } if constraint . operator () != IndexConstraintOp :: SQLITE_INDEX_CONSTRAINT_EQ { continue ; } if let CARRAY_COLUMN_POINTER = constraint . column () { ptr_idx = true ; constraint_usage . set_argv_index (1) ; constraint_usage . set_omit (true) ; } } if ptr_idx { info . set_estimated_cost (1_f64) ; info . set_estimated_rows (100) ; info . set_idx_num (1) ; } else { info . set_estimated_cost (2_147_483_647_f64) ; info . set_estimated_rows (2_147_483_647) ; info . set_idx_num (0) ; } Ok (()) } fn open (& mut self) -> Result < ArrayTabCursor < '_ > > { Ok (ArrayTabCursor :: new ()) } }
    };
}

impl_614!();
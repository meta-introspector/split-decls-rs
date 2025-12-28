macro_rules! deps {
    () => {
        IndexConstraintOp!();
        Result!();
        IndexInfo!();
        SeriesTab!();
        VTabConnection!();
        VTabConfig!();
        VTab!();
        SeriesTabCursor!();
    };
}

macro_rules! impl_637 {
    () => {
        deps!();
        unsafe impl < 'vtab > VTab < 'vtab > for SeriesTab { type Aux = () ; type Cursor = SeriesTabCursor < 'vtab > ; fn connect (db : & mut VTabConnection , _aux : Option < & () > , _args : & [& [u8]] ,) -> Result < (String , Self) > { let vtab = Self { base : ffi :: sqlite3_vtab :: default () , } ; db . config (VTabConfig :: Innocuous) ? ; Ok (("CREATE TABLE x(value,start hidden,stop hidden,step hidden)" . to_owned () , vtab ,)) } fn best_index (& self , info : & mut IndexInfo) -> Result < () > { let mut idx_num : QueryPlanFlags = QueryPlanFlags :: empty () ; let mut unusable_mask : QueryPlanFlags = QueryPlanFlags :: empty () ; let mut a_idx : [Option < usize > ; 3] = [None , None , None] ; for (i , constraint) in info . constraints () . enumerate () { if constraint . column () < SERIES_COLUMN_START { continue ; } let (i_col , i_mask) = match constraint . column () { SERIES_COLUMN_START => (0 , QueryPlanFlags :: START) , SERIES_COLUMN_STOP => (1 , QueryPlanFlags :: STOP) , SERIES_COLUMN_STEP => (2 , QueryPlanFlags :: STEP) , _ => { unreachable ! () } } ; if ! constraint . is_usable () { unusable_mask |= i_mask ; } else if constraint . operator () == IndexConstraintOp :: SQLITE_INDEX_CONSTRAINT_EQ { idx_num |= i_mask ; a_idx [i_col] = Some (i) ; } } let mut n_arg = 0 ; for j in a_idx . iter () . flatten () { n_arg += 1 ; let mut constraint_usage = info . constraint_usage (* j) ; constraint_usage . set_argv_index (n_arg) ; constraint_usage . set_omit (true) ; # [cfg (all (test , feature = "modern_sqlite"))] debug_assert_eq ! (Ok ("BINARY") , info . collation (* j)) ; } if ! (unusable_mask & ! idx_num) . is_empty () { return Err (error_from_sqlite_code (ffi :: SQLITE_CONSTRAINT , None)) ; } if idx_num . contains (QueryPlanFlags :: BOTH) { # [expect (clippy :: bool_to_int_with_if)] info . set_estimated_cost (f64 :: from (2 - if idx_num . contains (QueryPlanFlags :: STEP) { 1 } else { 0 } ,)) ; info . set_estimated_rows (1000) ; let order_by_consumed = { let mut order_bys = info . order_bys () ; if let Some (order_by) = order_bys . next () { if order_by . column () == 0 { if order_by . is_order_by_desc () { idx_num |= QueryPlanFlags :: DESC ; } else { idx_num |= QueryPlanFlags :: ASC ; } true } else { false } } else { false } } ; if order_by_consumed { info . set_order_by_consumed (true) ; } } else { info . set_estimated_rows (2_147_483_647) ; } info . set_idx_num (idx_num . bits ()) ; Ok (()) } fn open (& mut self) -> Result < SeriesTabCursor < '_ > > { Ok (SeriesTabCursor :: new ()) } }
    };
}

impl_637!();
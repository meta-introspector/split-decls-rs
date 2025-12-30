// Generated macro for impl_943 (impl)
macro_rules! Depcrate_vtab_vtablogimpl_943 {
() => {
// Module: crate::vtab::vtablog
// Provides: {"impl_943"}
// Dependencies: {}
unsafe impl < 'vtab > VTab < 'vtab > for VTabLog { type Aux = () ; type Cursor = VTabLogCursor < 'vtab > ; fn connect (db : & mut VTabConnection , aux : Option < & Self :: Aux > , args : & [& [u8]] ,) -> Result < (String , Self) > { Self :: connect_create (db , aux , args , false) } fn best_index (& self , info : & mut IndexInfo) -> Result < () > { println ! ("VTabLog::best_index({}, num_of_order_by: {}, col_used: {}, distinct: {:?})" , self . i_inst , info . num_of_order_by () , info . col_used () , info . distinct ()) ; let mut in_constraint = None ; for (i , constraint) in info . constraints () . enumerate () { println ! ("  constraint[{}]: col={}, usable={}, op={:?}, rhs={:?}, in={:?}" , i , constraint . column () , constraint . is_usable () , constraint . operator () , info . rhs_value (i) , info . is_in_constraint (i) ,) ; if info . is_in_constraint (i) ? { in_constraint = Some (i) ; } } info . set_estimated_cost (500.) ; info . set_estimated_rows (500) ; info . set_idx_str ("idx") ; info . set_idx_cstr (c"idx") ; if let Some (idx) = in_constraint { info . set_in_constraint (idx , true) ? ; info . constraint_usage (idx) . set_argv_index (1) ; } Ok (()) } fn open (& 'vtab mut self) -> Result < Self :: Cursor > { self . n_cursor += 1 ; println ! ("VTabLog::open(tab={}, cursor={})" , self . i_inst , self . n_cursor) ; Ok (VTabLogCursor { base : ffi :: sqlite3_vtab_cursor :: default () , i_cursor : self . n_cursor , row_id : 0 , phantom : PhantomData , }) } }
};
}

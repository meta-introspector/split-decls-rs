// Generated macro for impl_82 (impl)
macro_rules! Depcrate_rt_cellimpl_82 {
() => {
// Module: crate::rt::cell
// Provides: {"impl_82"}
// Dependencies: {}
impl Cell { pub (crate) fn new (location : Location) -> Cell { rt :: execution (| execution | { let state = State :: new (& execution . threads , location) ; Cell { state : execution . objects . insert (state) , } }) } pub (crate) fn start_read (& self , location : Location) -> Reading { rt :: synchronize (| execution | { let state = self . state . get_mut (& mut execution . objects) ; assert ! (! state . is_writing , "currently writing to cell") ; state . is_reading += 1 ; state . read_locations . track (location , & execution . threads) ; state . track_read (& execution . threads) ; Reading { state : self . state } }) } pub (crate) fn start_write (& self , location : Location) -> Writing { rt :: synchronize (| execution | { let state = self . state . get_mut (& mut execution . objects) ; assert ! (state . is_reading == 0 , "currently reading from cell") ; assert ! (! state . is_writing , "currently writing to cell") ; state . is_writing = true ; state . write_locations . track (location , & execution . threads) ; state . track_write (& execution . threads) ; Writing { state : self . state } }) } }
};
}

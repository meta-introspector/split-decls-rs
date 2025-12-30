// Generated macro for setup_clicker (function)
macro_rules! Depcratesetup_clicker {
() => {
// Module: crate
// Provides: {"setup_clicker"}
// Dependencies: {}
fn setup_clicker (document : & Document) { let num_clicks = document . get_element_by_id ("num-clicks") . expect ("should have #num-clicks on the page") ; let mut clicks = 0 ; let a = Closure :: < dyn FnMut () > :: new (move | | { clicks += 1 ; num_clicks . set_inner_html (& clicks . to_string ()) ; }) ; document . get_element_by_id ("green-square") . expect ("should have #green-square on the page") . dyn_ref :: < HtmlElement > () . expect ("#green-square be an `HtmlElement`") . set_onclick (Some (a . as_ref () . unchecked_ref ())) ; a . forget () ; }
};
}

// Generated macro for setup_clock (function)
macro_rules! Depcratesetup_clock {
() => {
// Module: crate
// Provides: {"setup_clock"}
// Dependencies: {}
fn setup_clock (window : & Window , document : & Document) -> Result < () , JsValue > { let current_time = document . get_element_by_id ("current-time") . expect ("should have #current-time on the page") ; update_time (& current_time) ; let a = Closure :: < dyn Fn () > :: new (move | | update_time (& current_time)) ; window . set_interval_with_callback_and_timeout_and_arguments_0 (a . as_ref () . unchecked_ref () , 1000) ? ; fn update_time (current_time : & Element) { current_time . set_inner_html (& String :: from (Date :: new_0 () . to_locale_string ("en-GB" , & JsValue :: undefined ()) ,)) ; } a . forget () ; Ok (()) }
};
}

// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
# [wasm_bindgen (start)] pub fn main () { console_error_panic_hook :: set_once () ; let document = web_sys :: window () . unwrap_throw () . document () . unwrap_throw () ; let el = document . get_element_by_id ("clock") . unwrap_throw () ; render_date (& el) ; spawn_local (async move { IntervalStream :: new (1_000) . for_each (| _ | { render_date (& el) ; ready (()) }) . await ; }) ; }
};
}

// Generated macro for leap_unwrap (macro)
macro_rules! Depcrateleap_unwrap {
() => {
// Module: crate
// Provides: {"leap_unwrap"}
// Dependencies: {}
macro_rules ! leap_unwrap { ($ x : expr) => { { match ($ x) { Some (val) => val , None => panic ! ("called `Option::unwrap()` on a `None` value") , } } } ; }
};
}

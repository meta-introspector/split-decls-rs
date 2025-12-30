// Generated macro for location (macro)
macro_rules! Depcrate_rt_locationlocation {
() => {
// Module: crate::rt::location
// Provides: {"location"}
// Dependencies: {}
macro_rules ! location { () => { { let enabled = crate :: rt :: execution (| execution | execution . location) ; if enabled { let location = crate :: rt :: Location :: from (std :: panic :: Location :: caller ()) ; location } else { crate :: rt :: Location :: disabled () } } } ; }
};
}

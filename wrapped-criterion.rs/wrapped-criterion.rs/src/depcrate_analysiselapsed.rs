// Generated macro for elapsed (macro)
macro_rules! Depcrate_analysiselapsed {
() => {
// Module: crate::analysis
// Provides: {"elapsed"}
// Dependencies: {}
macro_rules ! elapsed { ($ msg : expr , $ block : expr) => { { let start = :: std :: time :: Instant :: now () ; let out = $ block ; let elapsed = & start . elapsed () ; info ! ("{} took {}" , $ msg , crate :: format :: time (elapsed . as_nanos () as f64)) ; out } } ; }
};
}

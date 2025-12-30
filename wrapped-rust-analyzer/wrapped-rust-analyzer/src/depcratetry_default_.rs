// Generated macro for try_default_ (macro)
macro_rules! Depcratetry_default_ {
() => {
// Module: crate
// Provides: {"try_default_"}
// Dependencies: {}
# [doc (hidden)] macro_rules ! try_default_ { ($ it : expr $ (,) ?) => { match $ it { Some (it) => it , None => return Ok (Default :: default ()) , } } ; }
};
}

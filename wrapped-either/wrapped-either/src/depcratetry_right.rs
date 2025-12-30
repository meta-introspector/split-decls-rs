// Generated macro for try_right (macro)
macro_rules! Depcratetry_right {
() => {
// Module: crate
// Provides: {"try_right"}
// Dependencies: {}
# [doc = " Dual to [`try_left!`], see its documentation for more information."] # [macro_export] macro_rules ! try_right { ($ expr : expr) => { match $ expr { $ crate :: Left (err) => return $ crate :: Left (:: core :: convert :: From :: from (err)) , $ crate :: Right (val) => val , } } ; }
};
}

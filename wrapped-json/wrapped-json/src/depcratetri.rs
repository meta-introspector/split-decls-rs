// Generated macro for tri (macro)
macro_rules! Depcratetri {
() => {
// Module: crate
// Provides: {"tri"}
// Dependencies: {}
macro_rules ! tri { ($ e : expr $ (,) ?) => { match $ e { core :: result :: Result :: Ok (val) => val , core :: result :: Result :: Err (err) => return core :: result :: Result :: Err (err) , } } ; }
};
}

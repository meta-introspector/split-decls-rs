// Generated macro for map_either (macro)
macro_rules! Depcratemap_either {
() => {
// Module: crate
// Provides: {"map_either"}
// Dependencies: {}
macro_rules ! map_either { ($ value : expr , $ pattern : pat => $ result : expr) => { match $ value { Left ($ pattern) => Left ($ result) , Right ($ pattern) => Right ($ result) , } } ; }
};
}

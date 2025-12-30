// Generated macro for composite (macro)
macro_rules! Depcrate_util_rangeintcomposite {
() => {
// Module: crate::util::rangeint
// Provides: {"composite"}
// Dependencies: {}
macro_rules ! composite { (($ ($ name : ident) ,* $ (,) ?) => $ with : expr) => { { crate :: util :: rangeint :: composite ! (($ ($ name = $ name) ,*) => $ with) } } ; (($ ($ name : ident = $ rangeint : expr) ,* $ (,) ?) => $ with : expr) => { { # [cfg (not (debug_assertions))] { $ (let $ name = $ rangeint . val ;) * let val = $ with ; crate :: util :: rangeint :: Composite { val } } # [cfg (debug_assertions)] { let val = { $ (let $ name = $ rangeint . val ;) * $ with } ; let min = { $ (let $ name = $ rangeint . min ;) * $ with } ; let max = { $ (let $ name = $ rangeint . max ;) * $ with } ; crate :: util :: rangeint :: Composite { val , min , max } } } } ; }
};
}

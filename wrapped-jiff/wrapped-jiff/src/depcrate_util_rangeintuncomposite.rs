// Generated macro for uncomposite (macro)
macro_rules! Depcrate_util_rangeintuncomposite {
() => {
// Module: crate::util::rangeint
// Provides: {"uncomposite"}
// Dependencies: {}
macro_rules ! uncomposite { ($ composite : expr , $ val : ident => ($ ($ get : expr) ,* $ (,) ?) $ (,) ?) => { { # [cfg (not (debug_assertions))] { ($ ({ let val = { let $ val = $ composite . val ; $ get } ; crate :: util :: rangeint :: Composite { val } }) ,*) } # [cfg (debug_assertions)] { ($ ({ let val = { let $ val = $ composite . val ; $ get } ; let min = { let $ val = $ composite . min ; $ get } ; let max = { let $ val = $ composite . max ; $ get } ; crate :: util :: rangeint :: Composite { val , min , max } }) ,*) } } } ; }
};
}

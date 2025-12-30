// Generated macro for reduce (macro)
macro_rules! Depcrate_iter_multizipreduce {
() => {
// Module: crate::iter::multizip
// Provides: {"reduce"}
// Dependencies: {}
macro_rules ! reduce { ($ a : expr , $ b : expr , $ c : expr , $ d : expr , $ ($ x : expr) ,+ => $ fn : path) => { reduce ! (reduce ! ($ a , $ b , $ c , $ d => $ fn) , reduce ! ($ ($ x) ,+ => $ fn) => $ fn) } ; ($ a : expr , $ b : expr , $ ($ x : expr) ,+ => $ fn : path) => { reduce ! (reduce ! ($ a , $ b => $ fn) , reduce ! ($ ($ x) ,+ => $ fn) => $ fn) } ; ($ a : expr , $ b : expr => $ fn : path) => { $ fn ($ a , $ b) } ; ($ a : expr => $ fn : path) => { $ a } ; }
};
}

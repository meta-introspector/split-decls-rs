// Generated macro for forward (macro)
macro_rules! Depcrateforward {
() => {
// Module: crate
// Provides: {"forward"}
// Dependencies: {}
macro_rules ! forward { ($ ($ fn : ident ($ ($ param : ident : $ ty : ty) ,*) $ (-> $ ret : ty) ?) ,+ $ (,) ?) => { impl Build { $ (fn $ fn (& self , $ ($ param : $ ty) ,*) $ (-> $ ret) ? { self . config .$ fn ($ ($ param) ,*) }) + } } }
};
}

// Generated macro for supersuperfy_bounds (function)
macro_rules! Depcratesupersuperfy_bounds {
() => {
// Module: crate
// Provides: {"supersuperfy_bounds"}
// Dependencies: {}
fn supersuperfy_bounds (bounds : & mut Punctuated < TypeParamBound , Token ! [+] > , levels : usize) { for bound in bounds . iter_mut () { if let TypeParamBound :: Trait (tb) = bound { supersuperfy_path (& mut tb . path , levels) ; } } }
};
}

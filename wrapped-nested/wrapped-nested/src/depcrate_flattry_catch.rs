// Generated macro for try_catch (function)
macro_rules! Depcrate_flattry_catch {
() => {
// Module: crate::flat
// Provides: {"try_catch"}
// Dependencies: {}
fn try_catch < 'sval , T , S : Stream < 'sval > > (stream : & mut FlatStream < 'sval , S > , f : impl FnOnce (& mut FlatStream < 'sval , S >) -> Result < T > ,) -> sval :: Result < T > { match f (stream) { Ok (v) => Ok (v) , Err (e) => { stream . state = State :: Done (Some (Err (e))) ; sval :: error () } } }
};
}

// Generated macro for def_set_checker_func (macro)
macro_rules! Depcrate_drawing_backend_impl_mockeddef_set_checker_func {
() => {
// Module: crate::drawing::backend_impl::mocked
// Provides: {"def_set_checker_func"}
// Dependencies: {}
macro_rules ! def_set_checker_func { (drop_check , $ ($ param : ty) ,*) => { pub fn drop_check < T : FnMut ($ ($ param ,) *) + 'static > (& mut self , check : T) -> & mut Self { self . drop_check = Some (Box :: new (check)) ; self } } ; ($ name : ident , $ ($ param : ty) ,*) => { pub fn $ name < T : FnMut ($ ($ param ,) *) + 'static > (& mut self , check : T) -> & mut Self { self .$ name . push_back (Box :: new (check)) ; self } } }
};
}

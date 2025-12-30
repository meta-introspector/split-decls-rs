// Generated macro for impl_97 (impl)
macro_rules! Depcrate_borrow_tracker_stacked_borrows_diagnosticsimpl_97 {
() => {
// Module: crate::borrow_tracker::stacked_borrows::diagnostics
// Provides: {"impl_97"}
// Dependencies: {}
impl Creation { fn generate_diagnostic (& self) -> (String , SpanData) { let tag = self . retag . new_tag ; if let Some (perm) = self . retag . permission { (format ! ("{tag:?} was created by a {:?} retag at offsets {:?}" , perm , self . retag . range ,) , self . span . data () ,) } else { assert ! (self . retag . range . size == Size :: ZERO) ; (format ! ("{tag:?} would have been created here, but this is a zero-size retag ({:?}) so the tag in question does not exist anywhere" , self . retag . range ,) , self . span . data () ,) } } }
};
}

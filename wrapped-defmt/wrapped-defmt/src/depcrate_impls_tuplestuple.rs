// Generated macro for tuple (macro)
macro_rules! Depcrate_impls_tuplestuple {
() => {
// Module: crate::impls::tuples
// Provides: {"tuple"}
// Dependencies: {}
macro_rules ! tuple { ($ format : expr , ($ ($ name : ident) ,+)) => (impl <$ ($ name : Format) ,+> Format for ($ ($ name ,) +) where last_type ! ($ ($ name ,) +) : ? Sized { default_format ! () ; # [inline] fn _format_tag () -> Str { internp ! ($ format) } # [inline] # [allow (non_snake_case , unused_assignments)] fn _format_data (& self) { let ($ (ref $ name ,) +) = * self ; $ (export :: istr (&$ name :: _format_tag ()) ; $ name . _format_data () ;) + } }) }
};
}

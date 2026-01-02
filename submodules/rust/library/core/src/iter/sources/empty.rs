mkuse!{use crate :: iter :: { FusedIterator , TrustedLen } ;}
mkuse!{use crate :: { fmt , marker } ;}

macro_rules! empty_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function empty in module {}", module_path!());
    };
}

mkfn!{
    empty_introspect!();
    # [doc = " Creates an iterator that yields nothing."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " Basic usage:"] # [doc = ""] # [doc = " ```"] # [doc = " use std::iter;"] # [doc = ""] # [doc = " // this could have been an iterator over i32, but alas, it's just not."] # [doc = " let mut nope = iter::empty::<i32>();"] # [doc = ""] # [doc = " assert_eq!(None, nope.next());"] # [doc = " ```"] # [stable (feature = "iter_empty" , since = "1.2.0")] # [rustc_const_stable (feature = "const_iter_empty" , since = "1.32.0")] pub const fn empty < T > () -> Empty < T > { Empty (marker :: PhantomData) }
}
mkitem!{mkstruct!{# [doc = " An iterator that yields nothing."] # [doc = ""] # [doc = " This `struct` is created by the [`empty()`] function. See its documentation for more."] # [must_use = "iterators are lazy and do nothing unless consumed"] # [stable (feature = "iter_empty" , since = "1.2.0")] # [rustc_diagnostic_item = "IterEmpty"] pub struct Empty < T > (marker :: PhantomData < fn () -> T >) ;}}
mkitem!{mkimpl!{# [stable (feature = "core_impl_debug" , since = "1.9.0")] impl < T > fmt :: Debug for Empty < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("Empty") . finish () } }}}
mkitem!{mkimpl!{# [stable (feature = "iter_empty" , since = "1.2.0")] impl < T > Iterator for Empty < T > { type Item = T ; fn next (& mut self) -> Option < T > { None } fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (0)) } }}}
mkitem!{mkimpl!{# [stable (feature = "iter_empty" , since = "1.2.0")] impl < T > DoubleEndedIterator for Empty < T > { fn next_back (& mut self) -> Option < T > { None } }}}
mkitem!{mkimpl!{# [stable (feature = "iter_empty" , since = "1.2.0")] impl < T > ExactSizeIterator for Empty < T > { fn len (& self) -> usize { 0 } }}}
mkitem!{mkimpl!{# [unstable (feature = "trusted_len" , issue = "37572")] unsafe impl < T > TrustedLen for Empty < T > { }}}
mkitem!{mkimpl!{# [stable (feature = "fused" , since = "1.26.0")] impl < T > FusedIterator for Empty < T > { }}}
mkitem!{mkimpl!{# [stable (feature = "iter_empty" , since = "1.2.0")] impl < T > Clone for Empty < T > { fn clone (& self) -> Empty < T > { Empty (marker :: PhantomData) } }}}
mkitem!{# [stable (feature = "iter_empty" , since = "1.2.0")] # [rustc_const_unstable (feature = "const_default" , issue = "143894")] impl < T > const Default for Empty < T > { fn default () -> Empty < T > { Empty (marker :: PhantomData) } }}
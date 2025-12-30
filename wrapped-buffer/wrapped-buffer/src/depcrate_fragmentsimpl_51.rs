// Generated macro for impl_51 (impl)
macro_rules! Depcrate_fragmentsimpl_51 {
() => {
// Module: crate::fragments
// Provides: {"impl_51"}
// Dependencies: {}
impl < 'sval , T : ? Sized + Fragment > FragmentBuf < 'sval , T > { # [inline (always)] fn new (value : & 'sval T) -> Self { FragmentBuf { value : value . to_fragment () , } } # [inline (always)] fn push (& mut self , fragment : & 'sval T) -> Result < () , Error > { if self . value . can_replace () { self . value = fragment . to_fragment () ; Ok (()) } else { self . push_computed (fragment) } } # [inline (always)] fn push_computed (& mut self , fragment : & T) -> Result < () , Error > { # [cfg (feature = "alloc")] { Fragment :: extend (& mut self . value , fragment) ; Ok (()) } # [cfg (not (feature = "alloc"))] { let _ = fragment ; Err (Error :: no_alloc ("computed fragment")) } } # [inline (always)] fn as_borrowed_inner (& self) -> Option < & 'sval T > { # [cfg (feature = "alloc")] { match self . value { Cow :: Borrowed (value) => Some (value) , Cow :: Owned (_) => None , } } # [cfg (not (feature = "alloc"))] { Some (self . value) } } # [inline (always)] fn as_inner (& self) -> & T { # [cfg (feature = "alloc")] { & * self . value } # [cfg (not (feature = "alloc"))] { self . value } } # [cfg (feature = "alloc")] pub (crate) fn into_owned_in_place (& mut self) -> & mut FragmentBuf < 'static , T > { crate :: assert_static (Fragment :: into_owned_in_place (& mut self . value)) ; unsafe { mem :: transmute :: < & mut FragmentBuf < 'sval , T > , & mut FragmentBuf < 'static , T > > (self) } } }
};
}

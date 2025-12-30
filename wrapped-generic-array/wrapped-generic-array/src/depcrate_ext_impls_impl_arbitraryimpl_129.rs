// Generated macro for impl_129 (impl)
macro_rules! Depcrate_ext_impls_impl_arbitraryimpl_129 {
() => {
// Module: crate::ext_impls::impl_arbitrary
// Provides: {"impl_129"}
// Dependencies: {}
impl < 'a , T , N : ArrayLength > Arbitrary < 'a > for GenericArray < T , N > where T : Arbitrary < 'a > , { # [inline] fn arbitrary (u : & mut arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { GenericArray :: try_generate (| _ | T :: arbitrary (u)) } # [inline] fn arbitrary_take_rest (mut u : arbitrary :: Unstructured < 'a >) -> arbitrary :: Result < Self > { let mut array = Self :: arbitrary (& mut u) ? ; if let Some (last) = array . last_mut () { * last = T :: arbitrary_take_rest (u) ? ; } Ok (array) } fn size_hint (depth : usize) -> (usize , Option < usize >) { Self :: try_size_hint (depth) . unwrap_or_default () } fn try_size_hint (depth : usize ,) -> arbitrary :: Result < (usize , Option < usize >) , arbitrary :: MaxRecursionReached > { let hint = < T as Arbitrary > :: try_size_hint (depth) ? ; Ok (core :: iter :: repeat (hint) . take (N :: USIZE) . fold ((0 , Some (0)) , arbitrary :: size_hint :: and)) } }
};
}

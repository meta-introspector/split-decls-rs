// Generated macro for impl_60 (impl)
macro_rules! Depcrate_foreign_core_arrayimpl_60 {
() => {
// Module: crate::foreign::core::array
// Provides: {"impl_60"}
// Dependencies: {}
impl < 'a , T , const N : usize > Arbitrary < 'a > for [T ; N] where T : Arbitrary < 'a > , { # [inline] fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { try_create_array (| _ | < T as Arbitrary < 'a > > :: arbitrary (u)) } # [inline] fn arbitrary_take_rest (mut u : Unstructured < 'a >) -> Result < Self > { let mut array = Self :: arbitrary (& mut u) ? ; if let Some (last) = array . last_mut () { * last = Arbitrary :: arbitrary_take_rest (u) ? ; } Ok (array) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { Self :: try_size_hint (depth) . unwrap_or_default () } # [inline] fn try_size_hint (depth : usize) -> Result < (usize , Option < usize >) , crate :: MaxRecursionReached > { let hint = < T as Arbitrary > :: try_size_hint (depth) ? ; Ok (size_hint :: and_all (& array :: from_fn :: < _ , N , _ > (| _ | hint))) } }
};
}

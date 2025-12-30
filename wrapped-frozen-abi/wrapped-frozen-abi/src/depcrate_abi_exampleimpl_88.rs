// Generated macro for impl_88 (impl)
macro_rules! Depcrate_abi_exampleimpl_88 {
() => {
// Module: crate::abi_example
// Provides: {"impl_88"}
// Dependencies: {}
impl < T : AbiExample > AbiExample for std :: rc :: Rc < T > { fn example () -> Self { println ! ("AbiExample for (Rc<T>): {}" , type_name ::< Self > ()) ; std :: rc :: Rc :: new (T :: example ()) } }
};
}

// Generated macro for impl_89 (impl)
macro_rules! Depcrate_abi_exampleimpl_89 {
() => {
// Module: crate::abi_example
// Provides: {"impl_89"}
// Dependencies: {}
impl < T : AbiExample > AbiExample for std :: rc :: Weak < T > { fn example () -> Self { println ! ("AbiExample for (Rc's Weak<T>): {}" , type_name ::< Self > ()) ; std :: rc :: Rc :: downgrade (leak_and_inhibit_drop (std :: rc :: Rc :: new (T :: example ()))) } }
};
}

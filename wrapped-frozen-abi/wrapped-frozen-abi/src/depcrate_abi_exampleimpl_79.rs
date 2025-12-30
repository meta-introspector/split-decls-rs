// Generated macro for impl_79 (impl)
macro_rules! Depcrate_abi_exampleimpl_79 {
() => {
// Module: crate::abi_example
// Provides: {"impl_79"}
// Dependencies: {}
impl < T > AbiExample for Box < dyn Fn (& mut T) + Sync + Send > { fn example () -> Self { println ! ("AbiExample for (Box<T>): {}" , type_name ::< Self > ()) ; Box :: new (move | _t : & mut T | { }) } }
};
}

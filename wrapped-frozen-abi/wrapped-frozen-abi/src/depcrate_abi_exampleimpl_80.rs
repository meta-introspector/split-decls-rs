// Generated macro for impl_80 (impl)
macro_rules! Depcrate_abi_exampleimpl_80 {
() => {
// Module: crate::abi_example
// Provides: {"impl_80"}
// Dependencies: {}
impl < T , U > AbiExample for Box < dyn Fn (& mut T , U) + Sync + Send > { fn example () -> Self { println ! ("AbiExample for (Box<T, U>): {}" , type_name ::< Self > ()) ; Box :: new (move | _t : & mut T , _u : U | { }) } }
};
}

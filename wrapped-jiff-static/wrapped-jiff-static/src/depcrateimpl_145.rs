// Generated macro for impl_145 (impl)
macro_rules! Depcrateimpl_145 {
() => {
// Module: crate
// Provides: {"impl_145"}
// Dependencies: {}
impl TzifDateTime { fn quote (& self) -> proc_macro2 :: TokenStream { let year = self . year () ; let month = self . month () ; let day = self . day () ; let hour = self . hour () ; let minute = self . minute () ; let second = self . second () ; quote ! { jiff :: shared :: TzifDateTime :: new (# year , # month , # day , # hour , # minute , # second ,) } } }
};
}

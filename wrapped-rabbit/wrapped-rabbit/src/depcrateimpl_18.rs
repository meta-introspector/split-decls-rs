// Generated macro for impl_18 (impl)
macro_rules! Depcrateimpl_18 {
() => {
// Module: crate
// Provides: {"impl_18"}
// Dependencies: {}
impl KeyInit for RabbitKeyOnlyCore { fn new (key : & Key) -> Self { Self { state : State :: setup_key ((* key) . into ()) , } } }
};
}

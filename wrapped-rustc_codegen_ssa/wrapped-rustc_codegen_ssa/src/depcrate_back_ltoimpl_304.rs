// Generated macro for impl_304 (impl)
macro_rules! Depcrate_back_ltoimpl_304 {
() => {
// Module: crate::back::lto
// Provides: {"impl_304"}
// Dependencies: {}
impl < B : WriteBackendMethods > ThinModule < B > { pub fn name (& self) -> & str { self . shared . module_names [self . idx] . to_str () . unwrap () } pub fn cost (& self) -> u64 { self . data () . len () as u64 } pub fn data (& self) -> & [u8] { let a = self . shared . thin_buffers . get (self . idx) . map (| b | b . data ()) ; a . unwrap_or_else (| | { let len = self . shared . thin_buffers . len () ; self . shared . serialized_modules [self . idx - len] . data () }) } }
};
}

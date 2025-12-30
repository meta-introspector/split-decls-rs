// Generated macro for impl_15 (impl)
macro_rules! Depcrate_serde_implimpl_15 {
() => {
// Module: crate::serde_impl
// Provides: {"impl_15"}
// Dependencies: {}
impl < 'de , T : Deserialize < 'de > > Visitor < 'de > for AtomicLazyCellVisitor < T > { type Value = AtomicLazyCell < T > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("an AtomicLazyCell") } fn visit_some < D : Deserializer < 'de > > (self , deserializer : D) -> Result < Self :: Value , D :: Error > { let mut cell = AtomicLazyCell :: new () ; cell . replace (T :: deserialize (deserializer) ?) ; Ok (cell) } fn visit_none < E : de :: Error > (self) -> Result < Self :: Value , E > { Ok (AtomicLazyCell :: new ()) } }
};
}

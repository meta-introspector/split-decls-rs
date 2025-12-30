// Generated macro for impl_12 (impl)
macro_rules! Depcrate_serde_implimpl_12 {
() => {
// Module: crate::serde_impl
// Provides: {"impl_12"}
// Dependencies: {}
impl < 'de , T : Deserialize < 'de > > Visitor < 'de > for LazyCellVisitor < T > { type Value = LazyCell < T > ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("a LazyCell") } fn visit_some < D : Deserializer < 'de > > (self , deserializer : D) -> Result < Self :: Value , D :: Error > { let mut cell = LazyCell :: new () ; cell . replace (T :: deserialize (deserializer) ?) ; Ok (cell) } fn visit_none < E : de :: Error > (self) -> Result < Self :: Value , E > { Ok (LazyCell :: new ()) } }
};
}

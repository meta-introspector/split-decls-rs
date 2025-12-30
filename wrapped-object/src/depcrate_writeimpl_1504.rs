// Generated macro for impl_1504 (impl)
macro_rules! Depcrate_writeimpl_1504 {
() => {
// Module: crate::write
// Provides: {"impl_1504"}
// Dependencies: {}
impl StandardSection { # [doc = " Return the section kind of a standard section."] pub fn kind (self) -> SectionKind { match self { StandardSection :: Text => SectionKind :: Text , StandardSection :: Data => SectionKind :: Data , StandardSection :: ReadOnlyData => SectionKind :: ReadOnlyData , StandardSection :: ReadOnlyDataWithRel => SectionKind :: ReadOnlyDataWithRel , StandardSection :: ReadOnlyString => SectionKind :: ReadOnlyString , StandardSection :: UninitializedData => SectionKind :: UninitializedData , StandardSection :: Tls => SectionKind :: Tls , StandardSection :: UninitializedTls => SectionKind :: UninitializedTls , StandardSection :: TlsVariables => SectionKind :: TlsVariables , StandardSection :: Common => SectionKind :: Common , StandardSection :: GnuProperty => SectionKind :: Note , } } fn all () -> & 'static [StandardSection] { & [StandardSection :: Text , StandardSection :: Data , StandardSection :: ReadOnlyData , StandardSection :: ReadOnlyDataWithRel , StandardSection :: ReadOnlyString , StandardSection :: UninitializedData , StandardSection :: Tls , StandardSection :: UninitializedTls , StandardSection :: TlsVariables , StandardSection :: Common , StandardSection :: GnuProperty ,] } }
};
}

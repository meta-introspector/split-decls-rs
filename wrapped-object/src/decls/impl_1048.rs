macro_rules! deps {
    () => {
        SectionKind!();
        GnuProperty!();
        StandardSection!();
        Note!();
    };
}

macro_rules! impl_1048 {
    () => {
        deps!();
        impl StandardSection { # [doc = " Return the section kind of a standard section."] pub fn kind (self) -> SectionKind { match self { StandardSection :: Text => SectionKind :: Text , StandardSection :: Data => SectionKind :: Data , StandardSection :: ReadOnlyData => SectionKind :: ReadOnlyData , StandardSection :: ReadOnlyDataWithRel => SectionKind :: ReadOnlyDataWithRel , StandardSection :: ReadOnlyString => SectionKind :: ReadOnlyString , StandardSection :: UninitializedData => SectionKind :: UninitializedData , StandardSection :: Tls => SectionKind :: Tls , StandardSection :: UninitializedTls => SectionKind :: UninitializedTls , StandardSection :: TlsVariables => SectionKind :: TlsVariables , StandardSection :: Common => SectionKind :: Common , StandardSection :: GnuProperty => SectionKind :: Note , } } fn all () -> & 'static [StandardSection] { & [StandardSection :: Text , StandardSection :: Data , StandardSection :: ReadOnlyData , StandardSection :: ReadOnlyDataWithRel , StandardSection :: ReadOnlyString , StandardSection :: UninitializedData , StandardSection :: Tls , StandardSection :: UninitializedTls , StandardSection :: TlsVariables , StandardSection :: Common , StandardSection :: GnuProperty ,] } }
    };
}

impl_1048!();
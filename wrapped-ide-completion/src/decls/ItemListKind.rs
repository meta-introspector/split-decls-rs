macro_rules! ItemListKind {
    () => {
        # [doc = " The kind of item list a [`PathKind::Item`] belongs to."] # [derive (Debug , PartialEq , Eq)] pub (crate) enum ItemListKind { SourceFile , Module , Impl , TraitImpl (Option < ast :: Impl >) , Trait , ExternBlock { is_unsafe : bool } , }
    };
}

ItemListKind!();
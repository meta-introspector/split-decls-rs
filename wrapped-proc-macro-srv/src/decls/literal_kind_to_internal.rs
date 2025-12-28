macro_rules! literal_kind_to_internal {
    () => {
        fn literal_kind_to_internal (kind : bridge :: LitKind) -> tt :: LitKind { match kind { bridge :: LitKind :: Byte => tt :: LitKind :: Byte , bridge :: LitKind :: Char => tt :: LitKind :: Char , bridge :: LitKind :: Str => tt :: LitKind :: Str , bridge :: LitKind :: StrRaw (r) => tt :: LitKind :: StrRaw (r) , bridge :: LitKind :: ByteStr => tt :: LitKind :: ByteStr , bridge :: LitKind :: ByteStrRaw (r) => tt :: LitKind :: ByteStrRaw (r) , bridge :: LitKind :: CStr => tt :: LitKind :: CStr , bridge :: LitKind :: CStrRaw (r) => tt :: LitKind :: CStrRaw (r) , bridge :: LitKind :: Integer => tt :: LitKind :: Integer , bridge :: LitKind :: Float => tt :: LitKind :: Float , bridge :: LitKind :: ErrWithGuar => tt :: LitKind :: Err (()) , } }
    };
}

literal_kind_to_internal!()
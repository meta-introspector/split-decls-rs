macro_rules! deps {
    () => {
        IncorrectReprFormatGenericCause!();
    };
}

macro_rules! impl_340 {
    () => {
        deps!();
        impl IncorrectReprFormatGenericCause { pub (crate) fn from_lit_kind (span : Span , kind : & ast :: LitKind , name : Symbol) -> Option < Self > { match * kind { ast :: LitKind :: Int (value , ast :: LitIntType :: Unsuffixed) => { Some (Self :: Int { span , name , value : value . get () }) } ast :: LitKind :: Str (value , _) => Some (Self :: Symbol { span , name , value }) , _ => None , } } }
    };
}

impl_340!();
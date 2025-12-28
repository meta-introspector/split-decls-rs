macro_rules! BINARY_OPERATORS {
    () => {
        pub static BINARY_OPERATORS : & 'static [LangItem] = & [LangItem :: Add , LangItem :: Sub , LangItem :: Mul , LangItem :: Div , LangItem :: Rem , LangItem :: BitXor , LangItem :: BitAnd , LangItem :: BitOr , LangItem :: Shl , LangItem :: Shr , LangItem :: AddAssign , LangItem :: SubAssign , LangItem :: MulAssign , LangItem :: DivAssign , LangItem :: RemAssign , LangItem :: BitXorAssign , LangItem :: BitAndAssign , LangItem :: BitOrAssign , LangItem :: ShlAssign , LangItem :: ShrAssign , LangItem :: Index , LangItem :: IndexMut , LangItem :: PartialEq , LangItem :: PartialOrd ,] ;
    };
}

BINARY_OPERATORS!();
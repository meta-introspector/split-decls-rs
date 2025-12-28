macro_rules! OPERATORS {
    () => {
        pub static OPERATORS : & 'static [LangItem] = & [LangItem :: Add , LangItem :: Sub , LangItem :: Mul , LangItem :: Div , LangItem :: Rem , LangItem :: Neg , LangItem :: Not , LangItem :: BitXor , LangItem :: BitAnd , LangItem :: BitOr , LangItem :: Shl , LangItem :: Shr , LangItem :: AddAssign , LangItem :: SubAssign , LangItem :: MulAssign , LangItem :: DivAssign , LangItem :: RemAssign , LangItem :: BitXorAssign , LangItem :: BitAndAssign , LangItem :: BitOrAssign , LangItem :: ShlAssign , LangItem :: ShrAssign , LangItem :: Index , LangItem :: IndexMut , LangItem :: PartialEq , LangItem :: PartialOrd ,] ;
    };
}

OPERATORS!();
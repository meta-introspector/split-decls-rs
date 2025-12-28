macro_rules! deps {
    () => {
        Many0Count!();
        Verify!();
        Escaped!();
        Tag!();
        Char!();
        ManyMN!();
        Many1!();
        ManyTill!();
        Not!();
        Count!();
        Permutation!();
        Satisfy!();
        Many1Count!();
        LengthValue!();
        Float!();
        MapOpt!();
        EscapedTransform!();
        ErrorKind!();
        TakeWhileMN!();
        Complete!();
        TakeUntil!();
        MapRes!();
        Many0!();
        Fold!();
        Many!();
        Fail!();
    };
}

macro_rules! error_to_u32 {
    () => {
        deps!();
        # [rustfmt :: skip] # [allow (deprecated)] # [doc = " Converts an ErrorKind to a number"] pub fn error_to_u32 (e : & ErrorKind) -> u32 { match * e { ErrorKind :: Tag => 1 , ErrorKind :: MapRes => 2 , ErrorKind :: MapOpt => 3 , ErrorKind :: Alt => 4 , ErrorKind :: IsNot => 5 , ErrorKind :: IsA => 6 , ErrorKind :: SeparatedList => 7 , ErrorKind :: SeparatedNonEmptyList => 8 , ErrorKind :: Many1 => 9 , ErrorKind :: Count => 10 , ErrorKind :: TakeUntil => 12 , ErrorKind :: LengthValue => 15 , ErrorKind :: TagClosure => 16 , ErrorKind :: Alpha => 17 , ErrorKind :: Digit => 18 , ErrorKind :: AlphaNumeric => 19 , ErrorKind :: Space => 20 , ErrorKind :: MultiSpace => 21 , ErrorKind :: LengthValueFn => 22 , ErrorKind :: Eof => 23 , ErrorKind :: Switch => 27 , ErrorKind :: TagBits => 28 , ErrorKind :: OneOf => 29 , ErrorKind :: NoneOf => 30 , ErrorKind :: Char => 40 , ErrorKind :: CrLf => 41 , ErrorKind :: RegexpMatch => 42 , ErrorKind :: RegexpMatches => 43 , ErrorKind :: RegexpFind => 44 , ErrorKind :: RegexpCapture => 45 , ErrorKind :: RegexpCaptures => 46 , ErrorKind :: TakeWhile1 => 47 , ErrorKind :: Complete => 48 , ErrorKind :: Fix => 49 , ErrorKind :: Escaped => 50 , ErrorKind :: EscapedTransform => 51 , ErrorKind :: NonEmpty => 56 , ErrorKind :: ManyMN => 57 , ErrorKind :: HexDigit => 59 , ErrorKind :: OctDigit => 61 , ErrorKind :: Many0 => 62 , ErrorKind :: Not => 63 , ErrorKind :: Permutation => 64 , ErrorKind :: ManyTill => 65 , ErrorKind :: Verify => 66 , ErrorKind :: TakeTill1 => 67 , ErrorKind :: TakeWhileMN => 69 , ErrorKind :: TooLarge => 70 , ErrorKind :: Many0Count => 71 , ErrorKind :: Many1Count => 72 , ErrorKind :: Float => 73 , ErrorKind :: Satisfy => 74 , ErrorKind :: Fail => 75 , ErrorKind :: Many => 76 , ErrorKind :: Fold => 77 , ErrorKind :: BinDigit => 78 , ErrorKind :: Precedence => 79 , } }
    };
}

error_to_u32!();
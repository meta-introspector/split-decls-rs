macro_rules! deps {
    () => {
        Many!();
        Fail!();
        Many0!();
        EscapedTransform!();
        Count!();
        Verify!();
        MapRes!();
        Complete!();
        Tag!();
        TakeWhileMN!();
        Not!();
        Satisfy!();
        MapOpt!();
        LengthValue!();
        Fold!();
        Float!();
        TakeUntil!();
        ManyMN!();
        Char!();
        Many1!();
        ManyTill!();
        Escaped!();
        Many0Count!();
        Permutation!();
        Many1Count!();
    };
}

macro_rules! ErrorKind {
    () => {
        deps!();
        # [doc = " Indicates which parser returned an error"] # [rustfmt :: skip] # [derive (Debug , PartialEq , Eq , Hash , Clone , Copy)] # [allow (deprecated , missing_docs)] pub enum ErrorKind { Tag , MapRes , MapOpt , Alt , IsNot , IsA , SeparatedList , SeparatedNonEmptyList , Many0 , Many1 , ManyTill , Count , TakeUntil , LengthValue , TagClosure , Alpha , Digit , HexDigit , OctDigit , BinDigit , AlphaNumeric , Space , MultiSpace , LengthValueFn , Eof , Switch , TagBits , OneOf , NoneOf , Char , CrLf , RegexpMatch , RegexpMatches , RegexpFind , RegexpCapture , RegexpCaptures , TakeWhile1 , Complete , Fix , Escaped , EscapedTransform , NonEmpty , ManyMN , Not , Permutation , Verify , TakeTill1 , TakeWhileMN , TooLarge , Many0Count , Many1Count , Float , Satisfy , Fail , Many , Fold , Precedence , }
    };
}

ErrorKind!();
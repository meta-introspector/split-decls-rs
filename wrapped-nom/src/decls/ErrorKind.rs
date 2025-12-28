macro_rules! deps {
    () => {
        TakeUntil!();
        Permutation!();
        Many1Count!();
        EscapedTransform!();
        Fold!();
        Many1!();
        Tag!();
        Many!();
        Not!();
        Float!();
        MapRes!();
        ManyMN!();
        Verify!();
        Complete!();
        MapOpt!();
        Satisfy!();
        Count!();
        LengthValue!();
        TakeWhileMN!();
        Char!();
        ManyTill!();
        Many0!();
        Escaped!();
        Many0Count!();
        Fail!();
    };
}

macro_rules! ErrorKind {
    () => {
        deps!();
        # [doc = " Indicates which parser returned an error"] # [rustfmt :: skip] # [derive (Debug , PartialEq , Eq , Hash , Clone , Copy)] # [allow (deprecated , missing_docs)] pub enum ErrorKind { Tag , MapRes , MapOpt , Alt , IsNot , IsA , SeparatedList , SeparatedNonEmptyList , Many0 , Many1 , ManyTill , Count , TakeUntil , LengthValue , TagClosure , Alpha , Digit , HexDigit , OctDigit , BinDigit , AlphaNumeric , Space , MultiSpace , LengthValueFn , Eof , Switch , TagBits , OneOf , NoneOf , Char , CrLf , RegexpMatch , RegexpMatches , RegexpFind , RegexpCapture , RegexpCaptures , TakeWhile1 , Complete , Fix , Escaped , EscapedTransform , NonEmpty , ManyMN , Not , Permutation , Verify , TakeTill1 , TakeWhileMN , TooLarge , Many0Count , Many1Count , Float , Satisfy , Fail , Many , Fold , Precedence , }
    };
}

ErrorKind!()
macro_rules! deps {
    () => {
        PResult!();
        Compare!();
        Offset!();
        ParseError!();
        Parser!();
        ErrorKind!();
        Input!();
        Error!();
        OutputM!();
        AsChar!();
        OutputMode!();
        Float!();
        ParseTo!();
        Err!();
        AsBytes!();
        Emit!();
    };
}

macro_rules! impl_746 {
    () => {
        deps!();
        impl < I , O , E : ParseError < I > > Parser < I > for Float < O , E > where I : Clone + Offset , I : Input + crate :: traits :: ParseTo < O > + Compare < & 'static str > , < I as Input > :: Item : AsChar + Clone , I : AsBytes , I : for < 'a > Compare < & 'a [u8] > , { type Output = O ; type Error = E ; fn process < OM : crate :: OutputMode > (& mut self , input : I ,) -> crate :: PResult < OM , I , Self :: Output , Self :: Error > { let (i , s) = recognize_float_or_exceptions () . process :: < OutputM < Emit , OM :: Error , OM :: Incomplete > > (input) ? ; match s . parse_to () { Some (f) => Ok ((i , OM :: Output :: bind (| | f))) , None => Err (crate :: Err :: Error (OM :: Error :: bind (| | { E :: from_error_kind (i , crate :: error :: ErrorKind :: Float) }))) , } } }
    };
}

impl_746!();
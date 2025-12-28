macro_rules! deps {
    () => {
        Result!();
        Error!();
        Parser!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl < T : Parser > Parser for Box < T > { fn parse () -> Self { Box :: new (< T as Parser > :: parse ()) } fn try_parse () -> Result < Self , Error > { < T as Parser > :: try_parse () . map (Box :: new) } fn parse_from < I , It > (itr : I) -> Self where I : IntoIterator < Item = It > , It : Into < OsString > + Clone , { Box :: new (< T as Parser > :: parse_from (itr)) } fn try_parse_from < I , It > (itr : I) -> Result < Self , Error > where I : IntoIterator < Item = It > , It : Into < OsString > + Clone , { < T as Parser > :: try_parse_from (itr) . map (Box :: new) } }
    };
}

impl_21!()
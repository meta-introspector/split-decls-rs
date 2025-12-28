macro_rules! deps {
    () => {
        Stage!();
        AcceptContext!();
    };
}

macro_rules! parse_dialect {
    () => {
        deps!();
        fn parse_dialect < S : Stage > (cx : & mut AcceptContext < '_ , '_ , S > , dialect : Option < (Symbol , Span) > , failed : & mut bool ,) -> Option < (MirDialect , Span) > { let (dialect , span) = dialect ? ; let dialect = match dialect { sym :: analysis => MirDialect :: Analysis , sym :: built => MirDialect :: Built , sym :: runtime => MirDialect :: Runtime , _ => { cx . expected_specific_argument (span , & [sym :: analysis , sym :: built , sym :: runtime]) ; * failed = true ; return None ; } } ; Some ((dialect , span)) }
    };
}

parse_dialect!()
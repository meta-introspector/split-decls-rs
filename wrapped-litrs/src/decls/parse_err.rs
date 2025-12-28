macro_rules! deps {
    () => {
        StringLit!();
    };
}

macro_rules! parse_err {
    () => {
        deps!();
        # [test] fn parse_err () { assert_err ! (StringLit , r#"""# , UnterminatedString , None) ; assert_err ! (StringLit , r#""犬"# , UnterminatedString , None) ; assert_err ! (StringLit , r#""Jürgen"# , UnterminatedString , None) ; assert_err ! (StringLit , r#""foo bar baz"# , UnterminatedString , None) ; assert_err ! (StringLit , r#""fox"peter""# , InvalidSuffix , 5) ; assert_err ! (StringLit , r###"r#"foo "# bar"#"### , UnexpectedChar , 9) ; assert_err ! (StringLit , "\"\r\"" , CarriageReturn , 1) ; assert_err ! (StringLit , "\"fo\rx\"" , CarriageReturn , 3) ; assert_err ! (StringLit , "r\"\r\"" , CarriageReturn , 2) ; assert_err ! (StringLit , "r\"fo\rx\"" , CarriageReturn , 4) ; assert_err ! (StringLit , r##"r####""## , UnterminatedRawString , None) ; assert_err ! (StringLit , r#####"r##"foo"#bar"##### , UnterminatedRawString , None) ; assert_err ! (StringLit , r##"r####"## , InvalidLiteral , None) ; assert_err ! (StringLit , r##"r####x"## , InvalidLiteral , None) ; }
    };
}

parse_err!();
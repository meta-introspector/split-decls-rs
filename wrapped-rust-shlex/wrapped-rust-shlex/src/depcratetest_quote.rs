// Generated macro for test_quote (function)
macro_rules! Depcratetest_quote {
() => {
// Module: crate
// Provides: {"test_quote"}
// Dependencies: {}
# [test] # [cfg_attr (not (feature = "std") , allow (unreachable_code , unused_mut))] fn test_quote () { let tests = r#"
        <>                => <''>
        <foobar>          => <foobar>
        <foo bar>         => <'foo bar'>
        <"foo bar'">      => <"\"foo bar'\"">
        <'foo bar'>       => <"'foo bar'">
        <">               => <'"'>
        <"'>              => <"\"'">
        <hello!world>     => <'hello!world'>
        <'hello!world>    => <"'hello"'!world'>
        <'hello!>         => <"'hello"'!'>
        <hello ^ world>   => <'hello ''^ world'>
        <hello^>          => <hello'^'>
        <!world'>         => <'!world'"'">
        <{a, b}>          => <'{a, b}'>
        <NL>              => <'NL'>
        <^>               => <'^'>
        <foo^bar>         => <foo'^bar'>
        <NLx^>            => <'NLx''^'>
        <NL^x>            => <'NL''^x'>
        <NL ^x>           => <'NL ''^x'>
        <{a,b}>           => <'{a,b}'>
        <a,b>             => <'a,b'>
        <a..b             => <a..b>
        <'$>              => <"'"'$'>
        <"^>              => <'"''^'>
    "# ; let mut ok = true ; for test in tests . trim () . split ('\n') { let parts : Vec < String > = test . replace ("NL" , "\n") . split ("=>") . map (| part | part . trim () . trim_start_matches ('<') . trim_end_matches ('>') . to_owned ()) . collect () ; assert ! (parts . len () == 2) ; let unquoted = & * parts [0] ; let quoted_expected = & * parts [1] ; let quoted_actual = try_quote (& parts [0]) . unwrap () ; if quoted_expected != quoted_actual { # [cfg (not (feature = "std"))] panic ! ("FAIL: for input <{}>, expected <{}>, got <{}>" , unquoted , quoted_expected , quoted_actual) ; # [cfg (feature = "std")] println ! ("FAIL: for input <{}>, expected <{}>, got <{}>" , unquoted , quoted_expected , quoted_actual) ; ok = false ; } } assert ! (ok) ; }
};
}

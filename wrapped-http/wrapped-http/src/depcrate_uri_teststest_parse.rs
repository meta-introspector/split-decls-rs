// Generated macro for test_parse (macro)
macro_rules! Depcrate_uri_teststest_parse {
() => {
// Module: crate::uri::tests
// Provides: {"test_parse"}
// Dependencies: {}
macro_rules ! test_parse { ($ test_name : ident , $ str : expr , $ alt : expr , $ ($ method : ident = $ value : expr ,) *) => (# [test] fn $ test_name () { let orig_str = $ str ; let uri = match Uri :: from_str (orig_str) { Ok (uri) => uri , Err (err) => { panic ! ("parse error {:?} from {:?}" , err , orig_str) ; } , } ; $ (assert_eq ! (uri .$ method () , $ value , "{}: uri = {:?}" , stringify ! ($ method) , uri) ;) + assert_eq ! (uri , orig_str , "partial eq to original str") ; assert_eq ! (uri , uri . clone () , "clones are equal") ; let new_str = uri . to_string () ; let new_uri = Uri :: from_str (& new_str) . expect ("to_string output parses again as a Uri") ; assert_eq ! (new_uri , orig_str , "round trip still equals original str") ; const ALT : &'static [&'static str] = &$ alt ; for & alt in ALT . iter () { let other : Uri = alt . parse () . unwrap () ; assert_eq ! (uri , * alt) ; assert_eq ! (uri , other) ; } }) ; }
};
}

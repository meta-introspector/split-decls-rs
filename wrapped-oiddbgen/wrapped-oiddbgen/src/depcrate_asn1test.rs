// Generated macro for test (function)
macro_rules! Depcrate_asn1test {
() => {
// Module: crate::asn1
// Provides: {"test"}
// Dependencies: {}
# [test] fn test () { let asn1 = super :: Asn1Parser :: new (r"
            foo OBJECT IDENTIFIER ::= { bar(1) baz(2) 3 }
            bat OBJECT IDENTIFIER ::= { foo qux(4) 5 }
            quz OBJECT IDENTIFIER ::= { bat 6 }
        " ,) ; let answer = ("quz" . to_string () , "1.2.3.4.5.6" . to_string ()) ; let mut iter = asn1 . iter () ; assert_eq ! (Some (answer) , iter . next ()) ; assert_eq ! (None , iter . next ()) ; }
};
}

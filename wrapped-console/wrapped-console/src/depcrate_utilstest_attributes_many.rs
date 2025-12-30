// Generated macro for test_attributes_many (function)
macro_rules! Depcrate_utilstest_attributes_many {
() => {
// Module: crate::utils
// Provides: {"test_attributes_many"}
// Dependencies: {}
# [test] fn test_attributes_many () { let tests : [& [Attribute] ; 3] = [& [Attribute :: Bold , Attribute :: Underlined , Attribute :: BlinkFast , Attribute :: Hidden ,] , & [Attribute :: Dim , Attribute :: Italic , Attribute :: Blink , Attribute :: Reverse , Attribute :: StrikeThrough ,] , & Attribute :: MAP ,] ; for test_attrs in tests { let mut attrs = Attributes :: new () ; for attr in test_attrs { attrs = attrs . insert (* attr) ; } assert_eq ! (attrs . bits () . collect ::< Vec < _ >> () , test_attrs . iter () . map (| attr | * attr as u16) . collect ::< Vec < _ >> ()) ; assert_eq ! (& attrs . attrs () . collect ::< Vec < _ >> () , test_attrs) ; } }
};
}

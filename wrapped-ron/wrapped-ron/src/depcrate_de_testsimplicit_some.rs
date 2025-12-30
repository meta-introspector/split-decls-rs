// Generated macro for implicit_some (function)
macro_rules! Depcrate_de_testsimplicit_some {
() => {
// Module: crate::de::tests
// Provides: {"implicit_some"}
// Dependencies: {}
# [test] fn implicit_some () { use serde :: de :: DeserializeOwned ; fn de < T : DeserializeOwned > (s : & str) -> Option < T > { let enable = "#![enable(implicit_some)]\n" . to_string () ; super :: from_str :: < Option < T > > (& (enable + s)) . unwrap () } assert_eq ! (de ("'c'") , Some ('c')) ; assert_eq ! (de ("5") , Some (5)) ; assert_eq ! (de ("\"Hello\"") , Some ("Hello" . to_owned ())) ; assert_eq ! (de ("false") , Some (false)) ; assert_eq ! (de ("MyStruct(x: .4, y: .5)") , Some (MyStruct { x : 0.4 , y : 0.5 })) ; assert_eq ! (de ::< char > ("None") , None) ; assert_eq ! (de ::< Option < Option < char >>> ("None") , None) ; }
};
}

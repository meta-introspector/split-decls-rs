// Generated macro for impl_27 (impl)
macro_rules! Depcrate_utilsimpl_27 {
() => {
// Module: crate::utils
// Provides: {"impl_27"}
// Dependencies: {}
impl < S : AsRef < str > > TestResult < S > { fn assert (& self , output : impl AsRef < str >) { let regex = if self . exactly () { format ! ("test {}( - should panic)? ... {}" , self . name () , self . msg ()) } else { format ! ("test .*{}.*( - should panic)? ... {}" , self . name () , self . msg ()) } ; match self . times () { 0 => { } 1 => { assert_regex ! (regex , output . as_ref ()) ; } n => { assert_regex ! (regex , output . as_ref ()) ; assert_eq ! (n , output . count_regex (regex) , "test {} is present but wrong count" , self . name ()) ; } } } fn ok (name : S , exactly : bool , occurrence : usize) -> Self { Self :: Ok (name , TestInfo { exactly , times : occurrence , } ,) } fn fail (name : S , exactly : bool , occurrence : usize) -> Self { Self :: Fail (name , TestInfo { exactly , times : occurrence , } ,) } pub fn is_fail (& self) -> bool { use TestResult :: * ; matches ! (* self , Fail (_ , _)) } # [allow (dead_code)] pub fn is_ok (& self) -> bool { use self :: TestResult :: * ; matches ! (* self , Ok (_ , _)) } pub fn name (& self) -> String { use self :: TestResult :: * ; match * self { Ok (ref s , _) => s . as_ref () . to_owned () , Fail (ref s , _) => s . as_ref () . to_owned () , } } pub fn msg (& self) -> & 'static str { use self :: TestResult :: * ; match * self { Ok (_ , _) => "ok" , Fail (_ , _) => "FAILED" , } } fn info (& self) -> & TestInfo { match self { TestResult :: Ok (_ , o) => o , TestResult :: Fail (_ , o) => o , } } fn exactly (& self) -> bool { self . info () . exactly } fn times (& self) -> usize { self . info () . times } }
};
}

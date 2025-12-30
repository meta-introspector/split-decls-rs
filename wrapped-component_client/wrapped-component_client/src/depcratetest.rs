// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [test] fn test () -> Result < () > { let class = Class :: new () ? ; class . SetProperty (123) ? ; assert_eq ! (class . Property () ?, 123) ; assert_eq ! (class . Flags () ?, Flags :: Ok) ; let a = [1 , 2 , 3] ; let mut b = [0 ; 3] ; let mut c = Array :: new () ; let d = class . Int32Array (& a , & mut b , & mut c) ? ; assert_eq ! (a , b) ; assert_eq ! (a , c [..]) ; assert_eq ! (a , d [..]) ; let a = [HSTRING :: from ("a") , HSTRING :: from ("b") , HSTRING :: from ("c")] ; let mut b = [HSTRING :: new () , HSTRING :: new () , HSTRING :: new ()] ; let mut c = Array :: new () ; let d = class . StringArray (& a , & mut b , & mut c) ? ; assert_eq ! (a , b) ; assert_eq ! (a , c [..]) ; assert_eq ! (a , d [..]) ; let c : IStringable = Stringable . into () ; let d = Callback :: new (Ok) ; class . Input (& class , & class , & c , & d) ? ; assert ! (class . Input (None , None , None , None) . is_err ()) ; let inspectable : IInspectable = class . cast () ? ; assert_eq ! (inspectable . GetRuntimeClassName () ?, "test_component.Class") ; let inspectable : & IInspectable = unsafe { std :: mem :: transmute (& class) } ; assert_eq ! (inspectable . GetRuntimeClassName () ?, "test_component.IClass") ; Ok (()) }
};
}

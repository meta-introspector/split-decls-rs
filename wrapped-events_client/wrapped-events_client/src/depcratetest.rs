// Generated macro for test (function)
macro_rules! Depcratetest {
() => {
// Module: crate
// Provides: {"test"}
// Dependencies: {}
# [test] fn test () -> Result < () > { let class : & 'static Class = Box :: leak (Box :: new (Class :: new () ?)) ; assert_eq ! (0 , class . Signal (1) ?) ; let token = class . Event (& TypedEventHandler :: new (move | sender : Ref < Class > , args : Ref < i32 > | { assert_eq ! (sender . as_ref () . unwrap () , class) ; assert_eq ! (* args , 2) ; Ok (()) } ,)) ? ; assert_eq ! (1 , class . Signal (2) ?) ; class . RemoveEvent (token) ? ; assert_eq ! (0 , class . Signal (3) ?) ; class . Event (& TypedEventHandler :: new (move | sender : Ref < Class > , args : Ref < i32 > | { assert_eq ! (sender . as_ref () . unwrap () , class) ; assert_eq ! (* args , 4) ; Ok (()) } ,)) ? ; class . Event (& TypedEventHandler :: new (move | sender : Ref < Class > , args : Ref < i32 > | { assert_eq ! (sender . as_ref () . unwrap () , class) ; assert_eq ! (* args , 4) ; Ok (()) } ,)) ? ; assert_eq ! (2 , class . Signal (4) ?) ; Ok (()) }
};
}

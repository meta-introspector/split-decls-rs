// Generated macro for test_static (function)
macro_rules! Depcratetest_static {
() => {
// Module: crate
// Provides: {"test_static"}
// Dependencies: {}
# [test] fn test_static () -> Result < () > { assert_eq ! (0 , Class :: StaticSignal (1) ?) ; let token = Class :: StaticEvent (& EventHandler :: new (move | _ , args | { assert_eq ! (* args , 2) ; Ok (()) })) ? ; assert_eq ! (1 , Class :: StaticSignal (2) ?) ; Class :: RemoveStaticEvent (token) ? ; assert_eq ! (0 , Class :: StaticSignal (3) ?) ; Class :: StaticEvent (& EventHandler :: new (move | _ , args | { assert_eq ! (* args , 4) ; Ok (()) })) ? ; Class :: StaticEvent (& EventHandler :: new (move | _ , args | { assert_eq ! (* args , 4) ; Ok (()) })) ? ; assert_eq ! (2 , Class :: StaticSignal (4) ?) ; Ok (()) }
};
}

// Generated macro for check_strategy_sanity (function)
macro_rules! Depcrate_strategy_traitscheck_strategy_sanity {
() => {
// Module: crate::strategy::traits
// Provides: {"check_strategy_sanity"}
// Dependencies: {}
# [doc = " Run some tests on the given `Strategy` to ensure that it upholds the"] # [doc = " simplify/complicate contracts."] # [doc = ""] # [doc = " This is used to internally test proptest, but is made generally available"] # [doc = " for external implementations to use as well."] # [doc = ""] # [doc = " `options` can be passed to configure the test; if `None`, the defaults are"] # [doc = " used. Note that the defaults check for certain properties which are **not**"] # [doc = " actually required by the `Strategy` and `ValueTree` contracts; if you think"] # [doc = " your code is right but it fails the test, consider whether a non-default"] # [doc = " configuration is necessary."] # [doc = ""] # [doc = " This can work with fallible strategies, but limits how many times it will"] # [doc = " retry failures."] pub fn check_strategy_sanity < S : Strategy > (strategy : S , options : Option < CheckStrategySanityOptions > ,) where S :: Tree : Clone + fmt :: Debug , S :: Value : cmp :: PartialEq , { macro_rules ! assert_same { ($ a : expr , $ b : expr , $ ($ stuff : tt) *) => { { let a = $ a ; let b = $ b ; if a == a || b == b { assert_eq ! (a , b , $ ($ stuff) *) ; } } } } let options = options . unwrap_or_else (CheckStrategySanityOptions :: default) ; let mut config = Config :: default () ; if options . error_on_local_rejects { config . max_local_rejects = 0 ; } let mut runner = TestRunner :: new (config) ; for _ in 0 .. 1024 { let mut gen_tries = 0 ; let mut state ; loop { let err = match strategy . new_tree (& mut runner) { Ok (s) => { state = s ; break ; } Err (e) => e , } ; gen_tries += 1 ; if gen_tries > 100 { panic ! ("Strategy passed to check_strategy_sanity failed \
                     to generate a value over 100 times in a row; \
                     last failure reason: {}" , err) ; } } { let mut state = state . clone () ; let mut count = 0 ; while state . simplify () || state . complicate () { count += 1 ; if count > 65536 { panic ! ("Failed to converge on any value. State:\n{:#?}" , state) ; } } } let mut num_simplifies = 0 ; let mut before_simplified ; loop { before_simplified = state . clone () ; if ! state . simplify () { break ; } let mut complicated = state . clone () ; let before_complicated = state . clone () ; if options . strict_complicate_after_simplify { assert ! (complicated . complicate () , "complicate() returned false immediately after \
                     simplify() returned true. internal state after \
                     {} calls to simplify():\n\
                     {:#?}\n\
                     simplified to:\n\
                     {:#?}\n\
                     complicated to:\n\
                     {:#?}" , num_simplifies , before_simplified , state , complicated) ; } let mut prev_complicated = complicated . clone () ; let mut num_complications = 0 ; loop { if ! complicated . complicate () { break ; } prev_complicated = complicated . clone () ; num_complications += 1 ; if num_complications > 65_536 { panic ! ("complicate() returned true over 65536 times in a \
                         row; aborting due to possible infinite loop. \
                         If this is not an infinite loop, it may be \
                         necessary to reconsider how shrinking is \
                         implemented or use a simpler test strategy. \
                         Internal state:\n{:#?}" , state) ; } } assert_same ! (before_simplified . current () , complicated . current () , "Calling simplify(), then complicate() until it \
                 returned false, did not return to the value before \
                 simplify. Expected:\n\
                 {:#?}\n\
                 Actual:\n\
                 {:#?}\n\
                 Internal state after {} calls to simplify():\n\
                 {:#?}\n\
                 Internal state after another call to simplify():\n\
                 {:#?}\n\
                 Internal state after {} subsequent calls to \
                 complicate():\n\
                 {:#?}" , before_simplified . current () , complicated . current () , num_simplifies , before_simplified , before_complicated , num_complications + 1 , complicated) ; for iter in 1 .. 16 { assert_same ! (prev_complicated . current () , complicated . current () , "complicate() returned false but changed the output \
                     value anyway.\n\
                     Old value:\n\
                     {:#?}\n\
                     New value:\n\
                     {:#?}\n\
                     Old internal state:\n\
                     {:#?}\n\
                     New internal state after {} calls to complicate()\
                     including the :\n\
                     {:#?}" , prev_complicated . current () , complicated . current () , prev_complicated , iter , complicated) ; assert ! (! complicated . complicate () , "complicate() returned true after having returned \
                     false;\n\
                     Internal state before:\n{:#?}\n\
                     Internal state after calling complicate() {} times:\n\
                     {:#?}" , prev_complicated , iter + 1 , complicated) ; } num_simplifies += 1 ; if num_simplifies > 65_536 { panic ! ("simplify() returned true over 65536 times in a row, \
                     aborting due to possible infinite loop. If this is not \
                     an infinite loop, it may be necessary to reconsider \
                     how shrinking is implemented or use a simpler test \
                     strategy. Internal state:\n{:#?}" , state) ; } } for iter in 0 .. 16 { assert_same ! (before_simplified . current () , state . current () , "simplify() returned false but changed the output \
                 value anyway.\n\
                 Old value:\n\
                 {:#?}\n\
                 New value:\n\
                 {:#?}\n\
                 Previous internal state:\n\
                 {:#?}\n\
                 New internal state after calling simplify() {} times:\n\
                 {:#?}" , before_simplified . current () , state . current () , before_simplified , iter , state) ; if state . simplify () { panic ! ("simplify() returned true after having returned false. \
                     Previous internal state:\n\
                     {:#?}\n\
                     New internal state after calling simplify() {} times:\n\
                     {:#?}" , before_simplified , iter + 1 , state) ; } } } }
};
}

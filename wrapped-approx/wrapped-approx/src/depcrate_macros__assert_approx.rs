// Generated macro for __assert_approx (macro)
macro_rules! Depcrate_macros__assert_approx {
() => {
// Module: crate::macros
// Provides: {"__assert_approx"}
// Dependencies: {}
# [doc (hidden)] # [macro_export] macro_rules ! __assert_approx { ($ eq : ident , $ given : expr , $ expected : expr) => { { match (& ($ given) , & ($ expected)) { (given , expected) => assert ! ($ eq ! (* given , * expected) , "assert_{}!({}, {})

    left  = {:?}
    right = {:?}

" , stringify ! ($ eq) , stringify ! ($ given) , stringify ! ($ expected) , given , expected ,) , } } } ; ($ eq : ident , $ given : expr , $ expected : expr , $ ($ opt : ident = $ val : expr) ,+) => { { match (& ($ given) , & ($ expected)) { (given , expected) => assert ! ($ eq ! (* given , * expected , $ ($ opt = $ val) ,+) , "assert_{}!({}, {}, {})

    left  = {:?}
    right = {:?}

" , stringify ! ($ eq) , stringify ! ($ given) , stringify ! ($ expected) , stringify ! ($ ($ opt = $ val) ,+) , given , expected ,) , } } } ; }
};
}

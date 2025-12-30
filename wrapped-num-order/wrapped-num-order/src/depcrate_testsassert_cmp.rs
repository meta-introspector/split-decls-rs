// Generated macro for assert_cmp (function)
macro_rules! Depcrate_testsassert_cmp {
() => {
// Module: crate::tests
// Provides: {"assert_cmp"}
// Dependencies: {}
fn assert_cmp < T : Into < Option < Ordering > > > (lhs : & N , rhs : & N , expected : T) { # [derive (PartialEq)] struct Result { ord : Option < Ordering > , eq : bool , ne : bool , lt : bool , gt : bool , le : bool , ge : bool , } impl fmt :: Debug for Result { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { if let Some (ord) = self . ord { write ! (f , "<{:?} (" , ord) ? ; } else { write ! (f , "<_ (") ? ; } let neg = | b : bool | if b { "" } else { "!" } ; write ! (f , "{}eq {}ne {}lt {}gt {}le {}ge)>" , neg (self . eq) , neg (self . ne) , neg (self . lt) , neg (self . gt) , neg (self . le) , neg (self . gt)) } } let expected : Option < Ordering > = expected . into () ; let expected = match expected { Some (Less) => { Result { ord : expected , eq : false , ne : true , lt : true , gt : false , le : true , ge : false } } , Some (Equal) => { Result { ord : expected , eq : true , ne : false , lt : false , gt : false , le : true , ge : true } } , Some (Greater) => { Result { ord : expected , eq : false , ne : true , lt : false , gt : true , le : false , ge : true } } , None => { Result { ord : expected , eq : false , ne : true , lt : false , gt : false , le : false , ge : false } } , } ; let actual = repeat_arms ! { lhs ; x => { repeat_arms ! { rhs ; y => Result { ord : x . num_partial_cmp (y) , eq : x . num_eq (y) , ne : x . num_ne (y) , lt : x . num_lt (y) , gt : x . num_gt (y) , le : x . num_le (y) , ge : x . num_ge (y) } } } } ; assert_eq ! (expected , actual , "failed to compare {:?} against {:?}" , lhs , rhs) ; }
};
}

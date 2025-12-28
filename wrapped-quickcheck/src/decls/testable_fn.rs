macro_rules! deps {
    () => {
        Testable!();
        TestResult!();
        Arbitrary!();
        Gen!();
    };
}

macro_rules! testable_fn {
    () => {
        deps!();
        macro_rules ! testable_fn { ($ ($ name : ident) ,*) => { impl < T : Testable , $ ($ name : Arbitrary + Debug) ,*> Testable for fn ($ ($ name) ,*) -> T { # [allow (non_snake_case)] fn result (& self , g : & mut Gen) -> TestResult { fn shrink_failure < T : Testable , $ ($ name : Arbitrary + Debug) ,*> (g : & mut Gen , self_ : fn ($ ($ name) ,*) -> T , a : ($ ($ name ,) *) ,) -> Option < TestResult > { for t in a . shrink () { let ($ ($ name ,) *) = t . clone () ; let mut r_new = safe (move || { self_ ($ ($ name) ,*) }) . result (g) ; if r_new . is_failure () { { let ($ (ref $ name ,) *) : ($ ($ name ,) *) = t ; r_new . arguments = Some (debug_reprs (& [$ ($ name) ,*])) ; } let shrunk = shrink_failure (g , self_ , t) ; return Some (shrunk . unwrap_or (r_new)) } } None } let self_ = * self ; let a : ($ ($ name ,) *) = Arbitrary :: arbitrary (g) ; let ($ ($ name ,) *) = a . clone () ; let r = safe (move || { self_ ($ ($ name) ,*) }) . result (g) ; match r . status { Pass | Discard => r , Fail => { shrink_failure (g , self_ , a) . unwrap_or (r) } } } } } }
    };
}

testable_fn!()
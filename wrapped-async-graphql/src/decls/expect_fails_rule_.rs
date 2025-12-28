macro_rules! deps {
    () => {
        Visitor!();
    };
}

macro_rules! expect_fails_rule_ {
    () => {
        deps!();
        pub (crate) fn expect_fails_rule_ < 'a , V , F > (doc : & 'a ExecutableDocument , factory : F) where V : Visitor < 'a > + 'a , F : Fn () -> V , { if validate (doc , factory) . is_ok () { panic ! ("Expected rule to fail, but no errors were found") ; } }
    };
}

expect_fails_rule_!();
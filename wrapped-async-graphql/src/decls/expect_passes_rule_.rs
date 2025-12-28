macro_rules! deps {
    () => {
        Visitor!();
    };
}

macro_rules! expect_passes_rule_ {
    () => {
        deps!();
        pub (crate) fn expect_passes_rule_ < 'a , V , F > (doc : & 'a ExecutableDocument , factory : F) where V : Visitor < 'a > + 'a , F : Fn () -> V , { if let Err (errors) = validate (doc , factory) { for err in errors { if let Some (position) = err . locations . first () { print ! ("[{}:{}] " , position . line , position . column) ; } println ! ("{}" , err . message) ; } panic ! ("Expected rule to pass, but errors found") ; } }
    };
}

expect_passes_rule_!();
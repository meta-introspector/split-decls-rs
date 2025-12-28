macro_rules! unknown_const {
    () => {
        fn unknown_const (_ty : Ty < '_ >) -> Const < '_ > { Const :: new (DbInterner :: conjure () , ConstKind :: Error (ErrorGuaranteed)) }
    };
}

unknown_const!()
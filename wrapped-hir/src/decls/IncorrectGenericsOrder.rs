macro_rules! deps {
    () => {
        GenericArgKind!();
    };
}

macro_rules! IncorrectGenericsOrder {
    () => {
        deps!();
        # [derive (Debug)] pub struct IncorrectGenericsOrder { pub provided_arg : InFile < AstPtr < ast :: GenericArg > > , pub expected_kind : GenericArgKind , }
    };
}

IncorrectGenericsOrder!()
macro_rules! deps {
    () => {
        BuiltinUnpermittedTypeInitSub!();
    };
}

macro_rules! BuiltinUnpermittedTypeInit {
    () => {
        deps!();
        pub (crate) struct BuiltinUnpermittedTypeInit < 'a > { pub msg : DiagMessage , pub ty : Ty < 'a > , pub label : Span , pub sub : BuiltinUnpermittedTypeInitSub , pub tcx : TyCtxt < 'a > , }
    };
}

BuiltinUnpermittedTypeInit!()
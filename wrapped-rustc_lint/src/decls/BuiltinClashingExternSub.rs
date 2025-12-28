macro_rules! BuiltinClashingExternSub {
    () => {
        pub (crate) struct BuiltinClashingExternSub < 'a > { pub tcx : TyCtxt < 'a > , pub expected : Ty < 'a > , pub found : Ty < 'a > , }
    };
}

BuiltinClashingExternSub!()
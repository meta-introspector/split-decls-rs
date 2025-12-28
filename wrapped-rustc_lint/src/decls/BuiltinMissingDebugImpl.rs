macro_rules! BuiltinMissingDebugImpl {
    () => {
        pub (crate) struct BuiltinMissingDebugImpl < 'a > { pub tcx : TyCtxt < 'a > , pub def_id : DefId , }
    };
}

BuiltinMissingDebugImpl!()
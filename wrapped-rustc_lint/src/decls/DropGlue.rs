macro_rules! DropGlue {
    () => {
        pub (crate) struct DropGlue < 'a > { pub tcx : TyCtxt < 'a > , pub def_id : DefId , }
    };
}

DropGlue!();
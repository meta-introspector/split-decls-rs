macro_rules! deps {
    () => {
        FnCtxt!();
    };
}

macro_rules! InlineAsmCtxt {
    () => {
        deps!();
        pub (crate) struct InlineAsmCtxt < 'a , 'tcx > { target_features : & 'tcx FxIndexSet < Symbol > , fcx : & 'a FnCtxt < 'a , 'tcx > , }
    };
}

InlineAsmCtxt!();
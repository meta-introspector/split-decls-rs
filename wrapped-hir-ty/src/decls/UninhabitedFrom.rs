macro_rules! deps {
    () => {
        TraitEnvironment!();
    };
}

macro_rules! UninhabitedFrom {
    () => {
        deps!();
        struct UninhabitedFrom < 'a , 'db > { target_mod : ModuleId , recursive_ty : FxHashSet < Ty < 'db > > , max_depth : usize , infcx : & 'a InferCtxt < 'db > , env : Arc < TraitEnvironment < 'db > > , }
    };
}

UninhabitedFrom!();
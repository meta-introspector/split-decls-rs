macro_rules! VisibleTraits {
    () => {
        # [derive (Debug)] pub struct VisibleTraits (pub FxHashSet < TraitId >) ;
    };
}

VisibleTraits!()
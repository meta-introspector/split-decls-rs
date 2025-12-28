macro_rules! PickKind {
    () => {
        # [derive (Clone , Debug , PartialEq , Eq)] pub (crate) enum PickKind < 'tcx > { InherentImplPick , ObjectPick , TraitPick , WhereClausePick (ty :: PolyTraitRef < 'tcx > ,) , }
    };
}

PickKind!()
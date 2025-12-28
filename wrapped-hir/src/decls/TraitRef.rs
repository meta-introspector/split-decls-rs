macro_rules! TraitRef {
    () => {
        # [derive (Clone , PartialEq , Eq , Debug , Hash)] pub struct TraitRef < 'db > { env : Arc < TraitEnvironment < 'db > > , trait_ref : hir_ty :: next_solver :: TraitRef < 'db > , }
    };
}

TraitRef!()
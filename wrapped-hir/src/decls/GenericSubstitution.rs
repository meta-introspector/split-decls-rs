macro_rules! GenericSubstitution {
    () => {
        # [derive (Debug)] pub struct GenericSubstitution < 'db > { def : GenericDefId , subst : GenericArgs < 'db > , env : Arc < TraitEnvironment < 'db > > , }
    };
}

GenericSubstitution!()
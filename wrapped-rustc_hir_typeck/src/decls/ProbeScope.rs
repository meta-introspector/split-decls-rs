macro_rules! ProbeScope {
    () => {
        # [derive (PartialEq , Eq , Copy , Clone , Debug)] pub (crate) enum ProbeScope { Single (DefId) , TraitsInScope , AllTraits , }
    };
}

ProbeScope!();
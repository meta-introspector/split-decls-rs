macro_rules! deps {
    () => {
        ScrubbedTraitError!();
    };
}

macro_rules! impl_282 {
    () => {
        deps!();
        impl < 'tcx > ScrubbedTraitError < 'tcx > { pub fn is_true_error (& self) -> bool { match self { ScrubbedTraitError :: TrueError => true , ScrubbedTraitError :: Ambiguity | ScrubbedTraitError :: Cycle (_) => false , } } }
    };
}

impl_282!()
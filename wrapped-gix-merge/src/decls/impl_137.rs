macro_rules! deps {
    () => {
        PossibleConflict!();
    };
}

macro_rules! impl_137 {
    () => {
        deps!();
        impl PossibleConflict { pub (super) fn change_idx (& self) -> Option < usize > { match self { PossibleConflict :: TreeToNonTree { change_idx , .. } | PossibleConflict :: NonTreeToTree { change_idx , .. } => { * change_idx } PossibleConflict :: Match { change_idx , .. } | PossibleConflict :: PassedRewrittenDirectory { change_idx , .. } => Some (* change_idx) , } } }
    };
}

impl_137!()
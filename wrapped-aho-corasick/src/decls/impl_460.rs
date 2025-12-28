macro_rules! deps {
    () => {
        MatchKind!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        # [doc = " The default match kind is `MatchKind::Standard`."] impl Default for MatchKind { fn default () -> MatchKind { MatchKind :: Standard } }
    };
}

impl_460!()
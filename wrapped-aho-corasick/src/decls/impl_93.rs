macro_rules! deps {
    () => {
        Builder!();
        MatchKind!();
    };
}

macro_rules! impl_93 {
    () => {
        deps!();
        impl Default for Builder { fn default () -> Builder { Builder { match_kind : MatchKind :: default () , prefilter : true , ascii_case_insensitive : false , dense_depth : 3 , } } }
    };
}

impl_93!();
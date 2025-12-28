macro_rules! deps {
    () => {
        DiffLineKind!();
    };
}

macro_rules! impl_141 {
    () => {
        deps!();
        impl DiffLineKind { # [doc = " Returns a one-character representation for use in unified diffs."] pub const fn to_prefix (self) -> char { match self { DiffLineKind :: Context => ' ' , DiffLineKind :: Add => '+' , DiffLineKind :: Remove => '-' , } } }
    };
}

impl_141!()
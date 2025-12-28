macro_rules! deps {
    () => {
        SourceKind!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [doc = " Forwards to `Ord`"] impl PartialOrd for SourceKind { fn partial_cmp (& self , other : & SourceKind) -> Option < Ordering > { Some (self . cmp (other)) } }
    };
}

impl_30!();
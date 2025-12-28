macro_rules! deps {
    () => {
        Source!();
        SourceRef!();
    };
}

macro_rules! impl_25 {
    () => {
        deps!();
        impl SourceRef < '_ > { # [doc = " Create a fully owned instance by consuming this one."] pub fn into_owned (self) -> Source { match self { SourceRef :: ObjectId (id) => Source :: ObjectId (id) , SourceRef :: FullName (name) => Source :: FullName (name . into_owned () . into ()) , } } }
    };
}

impl_25!();
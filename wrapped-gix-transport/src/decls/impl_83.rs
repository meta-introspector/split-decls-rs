macro_rules! deps {
    () => {
        PostBodyDataKind!();
        WriteMode!();
    };
}

macro_rules! impl_83 {
    () => {
        deps!();
        impl From < WriteMode > for PostBodyDataKind { fn from (m : WriteMode) -> Self { match m { WriteMode :: Binary => PostBodyDataKind :: Unbounded , WriteMode :: OneLfTerminatedLinePerWriteCall => PostBodyDataKind :: BoundedAndFitsIntoMemory , } } }
    };
}

impl_83!();
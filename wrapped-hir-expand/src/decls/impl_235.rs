macro_rules! deps {
    () => {
        ExpandError!();
        ExpandErrorKind!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl From < mbe :: ExpandError > for ExpandError { fn from (mbe : mbe :: ExpandError) -> Self { ExpandError { inner : Arc :: new ((ExpandErrorKind :: Mbe (mbe . inner . 1 . clone ()) , mbe . inner . 0)) } } }
    };
}

impl_235!()
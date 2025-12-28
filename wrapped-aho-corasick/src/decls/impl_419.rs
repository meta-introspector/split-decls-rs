macro_rules! deps {
    () => {
        SmallIndexError!();
        SmallIndex!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        impl TryFrom < usize > for SmallIndex { type Error = SmallIndexError ; fn try_from (index : usize) -> Result < SmallIndex , SmallIndexError > { if index > SmallIndex :: MAX . as_usize () { return Err (SmallIndexError { attempted : index . as_u64 () }) ; } Ok (SmallIndex :: new_unchecked (index)) } }
    };
}

impl_419!()
macro_rules! deps {
    () => {
        SmallIndexError!();
        SmallIndex!();
    };
}

macro_rules! impl_417 {
    () => {
        deps!();
        impl TryFrom < u32 > for SmallIndex { type Error = SmallIndexError ; fn try_from (index : u32) -> Result < SmallIndex , SmallIndexError > { if index > SmallIndex :: MAX . as_u32 () { return Err (SmallIndexError { attempted : u64 :: from (index) }) ; } Ok (SmallIndex :: new_unchecked (index . as_usize ())) } }
    };
}

impl_417!();
macro_rules! deps {
    () => {
        UnblamedHunk!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl From < (Range < u32 > , ObjectId) > for UnblamedHunk { fn from (value : (Range < u32 > , ObjectId)) -> Self { let (range_in_blamed_file , suspect) = value ; let range_in_destination = range_in_blamed_file . clone () ; (range_in_blamed_file , suspect , range_in_destination) . into () } }
    };
}

impl_45!()
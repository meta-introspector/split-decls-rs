macro_rules! deps {
    () => {
        UnblamedHunk!();
    };
}

macro_rules! impl_46 {
    () => {
        deps!();
        impl From < (Range < u32 > , ObjectId , Range < u32 >) > for UnblamedHunk { fn from (value : (Range < u32 > , ObjectId , Range < u32 >)) -> Self { let (range_in_blamed_file , suspect , range_in_destination) = value ; assert ! (range_in_blamed_file . end > range_in_blamed_file . start , "{range_in_blamed_file:?}") ; assert ! (range_in_destination . end > range_in_destination . start , "{range_in_destination:?}") ; assert_eq ! (range_in_blamed_file . len () , range_in_destination . len ()) ; UnblamedHunk { range_in_blamed_file , suspects : [(suspect , range_in_destination)] . into () , source_file_name : None , } } }
    };
}

impl_46!();
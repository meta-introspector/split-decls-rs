macro_rules! deps {
    () => {
        BlameEntry!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl BlameEntry { # [doc = " Create a new instance."] pub fn new (range_in_blamed_file : Range < u32 > , range_in_source_file : Range < u32 > , commit_id : ObjectId , source_file_name : Option < BString > ,) -> Self { debug_assert ! (range_in_blamed_file . end > range_in_blamed_file . start , "{range_in_blamed_file:?}") ; debug_assert ! (range_in_source_file . end > range_in_source_file . start , "{range_in_source_file:?}") ; debug_assert_eq ! (range_in_source_file . len () , range_in_blamed_file . len ()) ; Self { start_in_blamed_file : range_in_blamed_file . start , start_in_source_file : range_in_source_file . start , len : NonZeroU32 :: new (range_in_blamed_file . len () as u32) . expect ("BUG: hunks are never empty") , commit_id , source_file_name , } } }
    };
}

impl_15!();
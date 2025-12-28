macro_rules! deps {
    () => {
        UnblamedHunk!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl UnblamedHunk { pub (crate) fn new (from_range_in_blamed_file : Range < u32 > , suspect : ObjectId) -> Self { let range_start = from_range_in_blamed_file . start ; let range_end = from_range_in_blamed_file . end ; UnblamedHunk { range_in_blamed_file : range_start .. range_end , suspects : [(suspect , range_start .. range_end)] . into () , source_file_name : None , } } pub (crate) fn has_suspect (& self , suspect : & ObjectId) -> bool { self . suspects . iter () . any (| entry | entry . 0 == * suspect) } pub (crate) fn get_range (& self , suspect : & ObjectId) -> Option < & Range < u32 > > { self . suspects . iter () . find (| entry | entry . 0 == * suspect) . map (| entry | & entry . 1) } }
    };
}

impl_20!();
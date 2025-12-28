macro_rules! deps {
    () => {
        BlameEntry!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl BlameEntry { # [doc = " Return the range of tokens this entry spans in the *Blamed File*."] pub fn range_in_blamed_file (& self) -> Range < usize > { let start = self . start_in_blamed_file as usize ; start .. start + self . len . get () as usize } # [doc = " Return the range of tokens this entry spans in the *Source File*."] pub fn range_in_source_file (& self) -> Range < usize > { let start = self . start_in_source_file as usize ; start .. start + self . len . get () as usize } }
    };
}

impl_16!();
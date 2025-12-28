macro_rules! deps {
    () => {
        Format!();
    };
}

macro_rules! GimliSectionMethods {
    () => {
        deps!();
        pub trait GimliSectionMethods { fn sleb (self , val : i64) -> Self ; fn uleb (self , val : u64) -> Self ; fn initial_length (self , format : Format , length : & Label , start : & Label) -> Self ; fn word (self , size : u8 , val : u64) -> Self ; fn word_label (self , size : u8 , val : & Label) -> Self ; }
    };
}

GimliSectionMethods!();
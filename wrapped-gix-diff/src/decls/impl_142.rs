macro_rules! deps {
    () => {
        HunkHeader!();
    };
}

macro_rules! impl_142 {
    () => {
        deps!();
        impl std :: fmt :: Display for HunkHeader { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "@@ -{},{} +{},{} @@" , self . before_hunk_start , self . before_hunk_len , self . after_hunk_start , self . after_hunk_len) } }
    };
}

impl_142!()
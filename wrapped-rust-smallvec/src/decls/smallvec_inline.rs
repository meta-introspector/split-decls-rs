macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! smallvec_inline {
    () => {
        deps!();
        # [macro_export] macro_rules ! smallvec_inline { (@ one $ x : expr) => (1usize) ; ($ elem : expr ; $ n : expr) => ({ $ crate :: SmallVec ::< _ , $ n >:: from_buf ([$ elem ; $ n]) }) ; ($ ($ x : expr) ,+ $ (,) ?) => ({ const N : usize = 0usize $ (+ $ crate :: smallvec_inline ! (@ one $ x)) *; $ crate :: SmallVec ::< _ , N >:: from_buf ([$ ($ x ,) *]) }) ; }
    };
}

smallvec_inline!()
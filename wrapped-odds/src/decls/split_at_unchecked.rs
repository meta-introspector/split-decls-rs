macro_rules! split_at_unchecked {
    () => {
        # [doc = " Unchecked version of `xs.split_at(i)`."] unsafe fn split_at_unchecked < T > (xs : & [T] , i : usize) -> (& [T] , & [T]) { (get_unchecked (xs , .. i) , get_unchecked (xs , i ..)) }
    };
}

split_at_unchecked!()
macro_rules! deps {
    () => {
        Regex!();
        OwnedDFA!();
    };
}

macro_rules! define_regex_type {
    () => {
        deps!();
        macro_rules ! define_regex_type { ($ (# [$ doc : meta]) *) => { # [cfg (feature = "alloc")] $ (# [$ doc]) * pub struct Regex < A = dense :: OwnedDFA > { forward : A , reverse : A , } # [cfg (not (feature = "alloc"))] $ (# [$ doc]) * pub struct Regex < A > { forward : A , reverse : A , } } ; }
    };
}

define_regex_type!()
macro_rules! deps {
    () => {
        Rng!();
    };
}

macro_rules! SampleString {
    () => {
        deps!();
        # [doc = " Sample or extend a [`String`]"] # [doc = ""] # [doc = " Helper methods to extend a [`String`] or sample a new [`String`]."] # [cfg (feature = "alloc")] pub trait SampleString { # [doc = " Append `len` random chars to `string`"] # [doc = ""] # [doc = " Note: implementations may leave `string` with excess capacity. If this"] # [doc = " is undesirable, consider calling [`String::shrink_to_fit`] after this"] # [doc = " method."] fn append_string < R : Rng + ? Sized > (& self , rng : & mut R , string : & mut String , len : usize) ; # [doc = " Generate a [`String`] of `len` random chars"] # [doc = ""] # [doc = " Note: implementations may leave the string with excess capacity. If this"] # [doc = " is undesirable, consider calling [`String::shrink_to_fit`] after this"] # [doc = " method."] # [inline] fn sample_string < R : Rng + ? Sized > (& self , rng : & mut R , len : usize) -> String { let mut s = String :: new () ; self . append_string (rng , & mut s , len) ; s } }
    };
}

SampleString!();
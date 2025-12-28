macro_rules! One {
    () => {
        # [doc = " One values: multiplicative identity element for `Self`."] pub trait One : ConstantTimeEq + Sized { # [doc = " Returns the multiplicative identity element of `Self`, `1`."] fn one () -> Self ; # [doc = " Determine if this value is equal to `1`."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " If one, returns `Choice(1)`. Otherwise, returns `Choice(0)`."] # [inline] fn is_one (& self) -> Choice { self . ct_eq (& Self :: one ()) } # [doc = " Set `self` to its multiplicative identity, i.e. `Self::one`."] # [inline] fn set_one (& mut self) { * self = One :: one () ; } # [doc = " Return the value `0` with the same precision as `other`."] fn one_like (other : & Self) -> Self where Self : Clone , { let mut ret = other . clone () ; ret . set_one () ; ret } }
    };
}

One!()
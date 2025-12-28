macro_rules! Zero {
    () => {
        # [doc = " Zero values: additive identity element for `Self`."] pub trait Zero : ConstantTimeEq + Sized { # [doc = " Returns the additive identity element of `Self`, `0`."] fn zero () -> Self ; # [doc = " Determine if this value is equal to `0`."] # [doc = ""] # [doc = " # Returns"] # [doc = ""] # [doc = " If zero, returns `Choice(1)`. Otherwise, returns `Choice(0)`."] # [inline] fn is_zero (& self) -> Choice { self . ct_eq (& Self :: zero ()) } # [doc = " Set `self` to its additive identity, i.e. `Self::zero`."] # [inline] fn set_zero (& mut self) { * self = Zero :: zero () ; } # [doc = " Return the value `0` with the same precision as `other`."] fn zero_like (other : & Self) -> Self where Self : Clone , { let mut ret = other . clone () ; ret . set_zero () ; ret } }
    };
}

Zero!()
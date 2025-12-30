// Generated macro for transmute_unchecked (function)
macro_rules! Depcrate_utilstransmute_unchecked {
() => {
// Module: crate::utils
// Provides: {"transmute_unchecked"}
// Dependencies: {}
# [doc = " Reinterprets the bits of a value of one type as another type."] # [doc = ""] # [doc = " Similar to [`std::mem::transmute`], except that it makes no compile-time"] # [doc = " guarantees about the layout of `T` or `U`, and is therefore even **more**"] # [doc = " dangerous than `transmute`. Extreme caution must be taken when using this"] # [doc = " function; it is up to the caller to assert that `T` and `U` have the same"] # [doc = " size and layout and that it is safe to do this conversion. Which it probably"] # [doc = " isn't, unless `T` and `U` are identical."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " This function panics if `T` and `U` have different sizes."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " It is up to the caller to uphold the following invariants:"] # [doc = ""] # [doc = " - `T` must have the same alignment as `U`"] # [doc = " - `T` must be safe to transmute into `U`"] # [inline (always)] pub (crate) unsafe fn transmute_unchecked < T , U > (value : T) -> U { assert ! (mem :: size_of ::< T > () == mem :: size_of ::< U > () , "cannot transmute_unchecked if Dst and Src have different size") ; let dest = ptr :: read (& value as * const T as * const U) ; mem :: forget (value) ; dest }
};
}

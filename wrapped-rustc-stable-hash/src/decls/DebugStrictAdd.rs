macro_rules! DebugStrictAdd {
    () => {
        # [doc = " Addition, but only overflow checked when `cfg(debug_assertions)` is set"] # [doc = " instead of respecting `-Coverflow-checks`."] # [doc = ""] # [doc = " This exists for performance reasons, as we ship rustc with overflow checks."] # [doc = " While overflow checks are perf neutral in almost all of the compiler, there"] # [doc = " are a few particularly hot areas where we don't want overflow checks in our"] # [doc = " dist builds. Overflow is still a bug there, so we want overflow check for"] # [doc = " builds with debug assertions."] # [doc = ""] # [doc = " That's a long way to say that this should be used in areas where overflow"] # [doc = " is a bug but overflow checking is too slow."] pub (crate) trait DebugStrictAdd { # [doc = " See [`DebugStrictAdd`]."] fn debug_strict_add (self , other : Self) -> Self ; }
    };
}

DebugStrictAdd!();
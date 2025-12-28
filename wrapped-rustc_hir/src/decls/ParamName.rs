macro_rules! ParamName {
    () => {
        # [derive (Debug , Copy , Clone , HashStable_Generic)] pub enum ParamName { # [doc = " Some user-given name like `T` or `'x`."] Plain (Ident) , # [doc = " Indicates an illegal name was given and an error has been"] # [doc = " reported (so we should squelch other derived errors)."] # [doc = ""] # [doc = " Occurs when, e.g., `'_` is used in the wrong place, or a"] # [doc = " lifetime name is duplicated."] Error (Ident) , # [doc = " Synthetic name generated when user elided a lifetime in an impl header."] # [doc = ""] # [doc = " E.g., the lifetimes in cases like these:"] # [doc = " ```ignore (fragment)"] # [doc = " impl Foo for &u32"] # [doc = " impl Foo<'_> for u32"] # [doc = " ```"] # [doc = " in that case, we rewrite to"] # [doc = " ```ignore (fragment)"] # [doc = " impl<'f> Foo for &'f u32"] # [doc = " impl<'f> Foo<'f> for u32"] # [doc = " ```"] # [doc = " where `'f` is something like `Fresh(0)`. The indices are"] # [doc = " unique per impl, but not necessarily continuous."] Fresh , }
    };
}

ParamName!();
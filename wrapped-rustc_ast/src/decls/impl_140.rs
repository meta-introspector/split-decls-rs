macro_rules! deps {
    () => {
        TraitObjectSyntax!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        # [doc = " SAFETY: `TraitObjectSyntax` only has 3 data-less variants which means"] # [doc = " it can be represented with a `u2`. We use `repr(u8)` to guarantee the"] # [doc = " discriminants of the variants are no greater than `3`."] unsafe impl Tag for TraitObjectSyntax { const BITS : u32 = 2 ; fn into_usize (self) -> usize { self as u8 as usize } unsafe fn from_usize (tag : usize) -> Self { match tag { 0 => TraitObjectSyntax :: Dyn , 1 => TraitObjectSyntax :: None , _ => unreachable ! () , } } }
    };
}

impl_140!()
macro_rules! deps {
    () => {
        Reader!();
        DebugRanges!();
    };
}

macro_rules! impl_553 {
    () => {
        deps!();
        impl < T > DebugRanges < T > { # [doc = " Create a `DebugRanges` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub (crate) fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugRanges < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
    };
}

impl_553!();
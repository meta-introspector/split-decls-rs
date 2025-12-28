macro_rules! deps {
    () => {
        DebugLoc!();
        Reader!();
    };
}

macro_rules! impl_443 {
    () => {
        deps!();
        impl < T > DebugLoc < T > { # [doc = " Create a `DebugLoc` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub (crate) fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugLoc < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
    };
}

impl_443!();
macro_rules! deps {
    () => {
        Reader!();
        DebugAddr!();
    };
}

macro_rules! impl_145 {
    () => {
        deps!();
        impl < T > DebugAddr < T > { # [doc = " Create a `DebugAddr` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugAddr < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
    };
}

impl_145!();
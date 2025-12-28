macro_rules! deps {
    () => {
        DebugStr!();
        Reader!();
    };
}

macro_rules! impl_585 {
    () => {
        deps!();
        impl < T > DebugStr < T > { # [doc = " Create a `DebugStr` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugStr < R > where F : FnMut (& 'a T) -> R , { borrow (& self . debug_str_section) . into () } }
    };
}

impl_585!();
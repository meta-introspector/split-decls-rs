macro_rules! deps {
    () => {
        DebugInfo!();
        Reader!();
    };
}

macro_rules! impl_610 {
    () => {
        deps!();
        impl < T > DebugInfo < T > { # [doc = " Create a `DebugInfo` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugInfo < R > where F : FnMut (& 'a T) -> R , { borrow (& self . debug_info_section) . into () } }
    };
}

impl_610!();
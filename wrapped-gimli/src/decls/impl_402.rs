macro_rules! deps {
    () => {
        Reader!();
        DebugLine!();
    };
}

macro_rules! impl_402 {
    () => {
        deps!();
        impl < T > DebugLine < T > { # [doc = " Create a `DebugLine` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugLine < R > where F : FnMut (& 'a T) -> R , { borrow (& self . debug_line_section) . into () } }
    };
}

impl_402!();
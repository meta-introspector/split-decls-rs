macro_rules! deps {
    () => {
        Reader!();
        DebugTypes!();
    };
}

macro_rules! impl_656 {
    () => {
        deps!();
        impl < T > DebugTypes < T > { # [doc = " Create a `DebugTypes` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugTypes < R > where F : FnMut (& 'a T) -> R , { borrow (& self . debug_types_section) . into () } }
    };
}

impl_656!();
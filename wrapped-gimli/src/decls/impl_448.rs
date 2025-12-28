macro_rules! deps {
    () => {
        Reader!();
        DebugLocLists!();
    };
}

macro_rules! impl_448 {
    () => {
        deps!();
        impl < T > DebugLocLists < T > { # [doc = " Create a `DebugLocLists` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub (crate) fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugLocLists < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
    };
}

impl_448!();
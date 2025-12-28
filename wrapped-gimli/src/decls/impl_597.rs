macro_rules! deps {
    () => {
        Reader!();
        DebugLineStr!();
    };
}

macro_rules! impl_597 {
    () => {
        deps!();
        impl < T > DebugLineStr < T > { # [doc = " Create a `DebugLineStr` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugLineStr < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
    };
}

impl_597!()
macro_rules! deps {
    () => {
        Reader!();
        DebugStrOffsets!();
    };
}

macro_rules! impl_590 {
    () => {
        deps!();
        impl < T > DebugStrOffsets < T > { # [doc = " Create a `DebugStrOffsets` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugStrOffsets < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
    };
}

impl_590!()
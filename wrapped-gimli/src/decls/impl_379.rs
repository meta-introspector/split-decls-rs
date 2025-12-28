macro_rules! deps {
    () => {
        Reader!();
        DebugCuIndex!();
    };
}

macro_rules! impl_379 {
    () => {
        deps!();
        impl < T > DebugCuIndex < T > { # [doc = " Create a `DebugCuIndex` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfPackageSections::borrow`."] pub (crate) fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugCuIndex < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
    };
}

impl_379!()
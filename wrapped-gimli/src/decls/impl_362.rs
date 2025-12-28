macro_rules! deps {
    () => {
        Reader!();
        DebugAranges!();
    };
}

macro_rules! impl_362 {
    () => {
        deps!();
        impl < T > DebugAranges < T > { # [doc = " Create a `DebugAranges` section that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `DwarfSections::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> DebugAranges < R > where F : FnMut (& 'a T) -> R , { borrow (& self . section) . into () } }
    };
}

impl_362!()
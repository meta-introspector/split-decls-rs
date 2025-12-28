macro_rules! deps {
    () => {
        LocationLists!();
        Reader!();
    };
}

macro_rules! impl_455 {
    () => {
        deps!();
        impl < T > LocationLists < T > { # [doc = " Create a `LocationLists` that references the data in `self`."] # [doc = ""] # [doc = " This is useful when `R` implements `Reader` but `T` does not."] # [doc = ""] # [doc = " Used by `Dwarf::borrow`."] pub fn borrow < 'a , F , R > (& 'a self , mut borrow : F) -> LocationLists < R > where F : FnMut (& 'a T) -> R , { LocationLists { debug_loc : borrow (& self . debug_loc . section) . into () , debug_loclists : borrow (& self . debug_loclists . section) . into () , } } }
    };
}

impl_455!();
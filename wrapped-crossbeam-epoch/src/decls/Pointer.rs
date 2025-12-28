macro_rules! deps {
    () => {
        Shared!();
        Pointable!();
        Owned!();
    };
}

macro_rules! Pointer {
    () => {
        deps!();
        # [doc = " A trait for either `Owned` or `Shared` pointers."] # [doc = ""] # [doc = " This trait is sealed and cannot be implemented for types outside of `crossbeam-epoch`."] pub trait Pointer < T : ? Sized + Pointable > : crate :: sealed :: Sealed { # [doc = " Returns the machine representation of the pointer."] fn into_ptr (self) -> * mut () ; # [doc = " Returns a new pointer pointing to the tagged pointer `data`."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The given `data` should have been created by `Pointer::into_ptr()`, and one `data` should"] # [doc = " not be converted back by `Pointer::from_ptr()` multiple times."] unsafe fn from_ptr (data : * mut ()) -> Self ; }
    };
}

Pointer!()
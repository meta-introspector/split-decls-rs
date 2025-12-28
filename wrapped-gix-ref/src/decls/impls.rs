macro_rules! deps {
    () => {
        PartialName!();
        PartialNameRef!();
    };
}

macro_rules! impls {
    () => {
        deps!();
        mod impls { use std :: borrow :: Borrow ; use crate :: { bstr :: ByteSlice , PartialName , PartialNameRef } ; impl Borrow < PartialNameRef > for PartialName { # [inline] fn borrow (& self) -> & PartialNameRef { PartialNameRef :: new_unchecked (self . 0 . as_bstr ()) } } impl AsRef < PartialNameRef > for PartialName { fn as_ref (& self) -> & PartialNameRef { self . borrow () } } impl ToOwned for PartialNameRef { type Owned = PartialName ; fn to_owned (& self) -> Self :: Owned { PartialName (self . 0 . to_owned ()) } } }
    };
}

impls!();
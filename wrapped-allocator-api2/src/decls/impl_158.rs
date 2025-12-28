macro_rules! deps {
    () => {
        Vec!();
        Allocator!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < T : PartialEq , A : Allocator > Vec < T , A > { # [doc = " Removes consecutive repeated elements in the vector according to the"] # [doc = " [`PartialEq`] trait implementation."] # [doc = ""] # [doc = " If the vector is sorted, this removes all duplicates."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::vec;"] # [doc = ""] # [doc = " let mut vec = vec![1, 2, 2, 3, 2];"] # [doc = ""] # [doc = " vec.dedup();"] # [doc = ""] # [doc = " assert_eq!(vec, [1, 2, 3, 2]);"] # [doc = " ```"] # [inline (always)] pub fn dedup (& mut self) { self . dedup_by (| a , b | a == b) } }
    };
}

impl_158!()
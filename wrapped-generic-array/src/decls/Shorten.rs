macro_rules! deps {
    () => {
        GenericSequence!();
        Lengthen!();
    };
}

macro_rules! Shorten {
    () => {
        deps!();
        # [doc = " Defines a `GenericSequence` which can be shortened by removing the first or last element from it."] # [doc = ""] # [doc = " Additionally, any shortened sequence can be lengthened by"] # [doc = " appending or prepending an element to it."] # [doc = ""] # [doc = " # Safety"] # [doc = " While the [`pop_back`](Shorten::pop_back) and [`pop_front`](Shorten::pop_front)"] # [doc = " methods are marked safe, care must be taken when implementing them."] pub unsafe trait Shorten < T > : Sized + GenericSequence < T > { # [doc = " `GenericSequence` that has one less element than `Self`"] type Shorter : Lengthen < T , Longer = Self > ; # [doc = " Returns a new array without the last element, and the last element."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use generic_array::{arr, sequence::Shorten};"] # [doc = ""] # [doc = " let a = arr![1, 2, 3, 4];"] # [doc = ""] # [doc = " let (init, last) = a.pop_back();"] # [doc = ""] # [doc = " assert_eq!(init, arr![1, 2, 3]);"] # [doc = " assert_eq!(last, 4);"] # [doc = " ```"] fn pop_back (self) -> (Self :: Shorter , T) ; # [doc = " Returns a new array without the first element, and the first element."] # [doc = " Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use generic_array::{arr, sequence::Shorten};"] # [doc = ""] # [doc = " let a = arr![1, 2, 3, 4];"] # [doc = ""] # [doc = " let (head, tail) = a.pop_front();"] # [doc = ""] # [doc = " assert_eq!(head, 1);"] # [doc = " assert_eq!(tail, arr![2, 3, 4]);"] # [doc = " ```"] fn pop_front (self) -> (T , Self :: Shorter) ; }
    };
}

Shorten!();
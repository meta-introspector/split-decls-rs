macro_rules! deps {
    () => {
        Shorten!();
        GenericSequence!();
    };
}

macro_rules! Lengthen {
    () => {
        deps!();
        # [doc = " Defines any `GenericSequence` which can be lengthened or extended by appending"] # [doc = " or prepending an element to it."] # [doc = ""] # [doc = " Any lengthened sequence can be shortened back to the original using `pop_front` or `pop_back`"] # [doc = ""] # [doc = " # Safety"] # [doc = " While the [`append`](Lengthen::append) and [`prepend`](Lengthen::prepend)"] # [doc = " methods are marked safe, care must be taken when implementing them."] pub unsafe trait Lengthen < T > : Sized + GenericSequence < T > { # [doc = " `GenericSequence` that has one more element than `Self`"] type Longer : Shorten < T , Shorter = Self > ; # [doc = " Returns a new array with the given element appended to the end of it."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use generic_array::{arr, sequence::Lengthen};"] # [doc = ""] # [doc = " let a = arr![1, 2, 3];"] # [doc = ""] # [doc = " let b = a.append(4);"] # [doc = ""] # [doc = " assert_eq!(b, arr![1, 2, 3, 4]);"] # [doc = " ```"] fn append (self , last : T) -> Self :: Longer ; # [doc = " Returns a new array with the given element prepended to the front of it."] # [doc = ""] # [doc = " Example:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use generic_array::{arr, sequence::Lengthen};"] # [doc = ""] # [doc = " let a = arr![1, 2, 3];"] # [doc = ""] # [doc = " let b = a.prepend(4);"] # [doc = ""] # [doc = " assert_eq!(b, arr![4, 1, 2, 3]);"] # [doc = " ```"] fn prepend (self , first : T) -> Self :: Longer ; }
    };
}

Lengthen!();
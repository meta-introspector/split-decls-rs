macro_rules! VecExt {
    () => {
        # [doc = " Extra methods for `Vec<T>`"] # [doc = ""] # [doc = " Requires `feature=\"std-vec\"`"] pub trait VecExt < T > { # [doc = " Retains only the elements specified by the predicate."] # [doc = ""] # [doc = " In other words, remove all elements `e` such that `f(&mut e)` returns false."] # [doc = " This method operates in place and preserves the order of the retained"] # [doc = " elements."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use odds::vec::VecExt;"] # [doc = " let mut vec = vec![1, 2, 3, 4];"] # [doc = " vec.retain_mut(|x| {"] # [doc = "     let keep = *x % 2 == 0;"] # [doc = "     *x *= 10;"] # [doc = "     keep"] # [doc = " });"] # [doc = " assert_eq!(vec, [20, 40]);"] # [doc = " ```"] fn retain_mut < F > (& mut self , f : F) where F : FnMut (& mut T) -> bool ; }
    };
}

VecExt!()
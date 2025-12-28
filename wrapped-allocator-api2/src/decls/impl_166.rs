macro_rules! deps {
    () => {
        Allocator!();
        Vec!();
        IntoIter!();
    };
}

macro_rules! impl_166 {
    () => {
        deps!();
        impl < T , A : Allocator > IntoIterator for Vec < T , A > { type Item = T ; type IntoIter = IntoIter < T , A > ; # [doc = " Creates a consuming iterator, that is, one that moves each value out of"] # [doc = " the vector (from start to end). The vector cannot be used after calling"] # [doc = " this."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::vec;"] # [doc = ""] # [doc = " let v = vec![\"a\".to_string(), \"b\".to_string()];"] # [doc = " let mut v_iter = v.into_iter();"] # [doc = ""] # [doc = " let first_element: Option<String> = v_iter.next();"] # [doc = ""] # [doc = " assert_eq!(first_element, Some(\"a\".to_string()));"] # [doc = " assert_eq!(v_iter.next(), Some(\"b\".to_string()));"] # [doc = " assert_eq!(v_iter.next(), None);"] # [doc = " ```"] # [inline (always)] fn into_iter (self) -> Self :: IntoIter { unsafe { let mut me = ManuallyDrop :: new (self) ; let alloc = ManuallyDrop :: new (ptr :: read (me . allocator ())) ; let begin = me . as_mut_ptr () ; let end = if size_of :: < T > () == 0 { begin . cast :: < u8 > () . wrapping_add (me . len ()) . cast () } else { begin . add (me . len ()) as * const T } ; let cap = me . buf . capacity () ; IntoIter { buf : NonNull :: new_unchecked (begin) , phantom : PhantomData , cap , alloc , ptr : begin , end , } } } }
    };
}

impl_166!();
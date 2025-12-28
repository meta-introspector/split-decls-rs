macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_135 {
    () => {
        deps!();
        impl < 'h > Iter < 'h > { # [doc = " Create a new generic memchr iterator."] # [inline (always)] pub (crate) fn new (haystack : & 'h [u8]) -> Iter < 'h > { Iter { original_start : haystack . as_ptr () , start : haystack . as_ptr () , end : haystack . as_ptr () . wrapping_add (haystack . len ()) , haystack : core :: marker :: PhantomData , } } # [doc = " Returns the next occurrence in the forward direction."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that if a pointer is returned from the closure"] # [doc = " provided, then it must be greater than or equal to the start pointer"] # [doc = " and less than the end pointer."] # [inline (always)] pub (crate) unsafe fn next (& mut self , mut find_raw : impl FnMut (* const u8 , * const u8) -> Option < * const u8 > ,) -> Option < usize > { let found = find_raw (self . start , self . end) ? ; let result = found . distance (self . original_start) ; self . start = found . add (1) ; Some (result) } # [doc = " Returns the number of remaining elements in this iterator."] # [inline (always)] pub (crate) fn count (self , mut count_raw : impl FnMut (* const u8 , * const u8) -> usize ,) -> usize { count_raw (self . start , self . end) } # [doc = " Returns the next occurrence in reverse."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that if a pointer is returned from the closure"] # [doc = " provided, then it must be greater than or equal to the start pointer"] # [doc = " and less than the end pointer."] # [inline (always)] pub (crate) unsafe fn next_back (& mut self , mut rfind_raw : impl FnMut (* const u8 , * const u8) -> Option < * const u8 > ,) -> Option < usize > { let found = rfind_raw (self . start , self . end) ? ; let result = found . distance (self . original_start) ; self . end = found ; Some (result) } # [doc = " Provides an implementation of `Iterator::size_hint`."] # [inline (always)] pub (crate) fn size_hint (& self) -> (usize , Option < usize >) { (0 , Some (self . end . as_usize () . saturating_sub (self . start . as_usize ()))) } }
    };
}

impl_135!();
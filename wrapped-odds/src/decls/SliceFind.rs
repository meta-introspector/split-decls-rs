macro_rules! SliceFind {
    () => {
        # [doc = " Element-finding methods for slices"] pub trait SliceFind { type Item ; # [doc = " Linear search for the first occurrence  `elt` in the slice."] # [doc = ""] # [doc = " Return its index if it is found, or None."] fn find < U : ? Sized > (& self , elt : & U) -> Option < usize > where Self :: Item : PartialEq < U > ; # [doc = " Linear search for the last occurrence  `elt` in the slice."] # [doc = ""] # [doc = " Return its index if it is found, or None."] fn rfind < U : ? Sized > (& self , elt : & U) -> Option < usize > where Self :: Item : PartialEq < U > ; }
    };
}

SliceFind!()
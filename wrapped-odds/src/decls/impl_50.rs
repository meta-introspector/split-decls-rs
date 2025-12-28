macro_rules! deps {
    () => {
        RevSlice!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < T > RevSlice < T > { # [doc = " Return the length of the slice."] pub fn len (& self) -> usize { self . 0 . len () } # [inline] fn raw_index_no_wrap (& self , i : usize) -> usize { self . len () - (1 + i) } # [doc = " Return the index into the underlying slice, if it's in bounds"] fn raw_index (& self , i : usize) -> Option < usize > { if i < self . len () { Some (self . raw_index_no_wrap (i)) } else { None } } # [doc = " Get element at index `i`."] # [doc = ""] # [doc = " See also indexing notation: `&foo[i]`."] pub fn get (& self , i : usize) -> Option < & T > { unsafe { self . raw_index (i) . map (move | ri | get_unchecked (& self . 0 , ri)) } } # [doc = " Get element at index `i`."] # [doc = ""] # [doc = " See also indexing notation: `&mut foo[i]`."] pub fn get_mut (& mut self , i : usize) -> Option < & mut T > { unsafe { self . raw_index (i) . map (move | ri | get_unchecked_mut (& mut self . 0 , ri)) } } pub fn inner_ref (& self) -> & [T] { & self . 0 } pub fn inner_mut (& mut self) -> & mut [T] { & mut self . 0 } # [cfg (feature = "std")] pub fn into_boxed_slice (self : Box < Self >) -> Box < [T] > { unsafe { transmute (self) } } # [doc = " Return a by-reference iterator"] pub fn iter (& self) -> Rev < Iter < T > > { self . into_iter () } # [doc = " Return a by-mutable-reference iterator"] pub fn iter_mut (& mut self) -> Rev < IterMut < T > > { self . into_iter () } pub fn split_at (& self , i : usize) -> (& Self , & Self) { assert ! (i <= self . len ()) ; let ri = self . len () - i ; let (a , b) = self . 0 . split_at (ri) ; (< _ > :: from (b) , < _ > :: from (a)) } pub fn split_at_mut (& mut self , i : usize) -> (& mut Self , & mut Self) { assert ! (i <= self . len ()) ; let ri = self . len () - i ; let (a , b) = self . 0 . split_at_mut (ri) ; (< _ > :: from (b) , < _ > :: from (a)) } }
    };
}

impl_50!()
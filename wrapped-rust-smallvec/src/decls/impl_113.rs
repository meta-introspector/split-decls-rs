macro_rules! deps {
    () => {
        SmallVec!();
    };
}

macro_rules! impl_113 {
    () => {
        deps!();
        impl < T : Clone , const N : usize > SmallVec < T , N > { # [inline] pub fn resize (& mut self , len : usize , value : T) { let old_len = self . len () ; if len > old_len { self . extend (core :: iter :: repeat_n (value , len - old_len)) ; } else { self . truncate (len) ; } } # [inline] pub fn extend_from_slice (& mut self , other : & [T]) { self . extend (other . iter ()) } pub fn extend_from_within < R > (& mut self , src : R) where R : core :: ops :: RangeBounds < usize > , { let src = slice_range (src , .. self . len ()) ; self . reserve (src . len ()) ; unsafe { # [cfg (feature = "specialization")] { < Self as spec_traits :: SpecExtendFromWithin < T > > :: spec_extend_from_within (self , src) ; } # [cfg (not (feature = "specialization"))] { self . extend_from_within_fallback (src) ; } } } # [inline] pub fn extend_from_slice_copy (& mut self , other : & [T]) where T : Copy , { let len = other . len () ; let src = other . as_ptr () ; let l = self . len () ; self . reserve (len) ; unsafe { let dst = self . as_mut_ptr () . add (l) ; copy_nonoverlapping (src , dst , len) ; self . set_len (l + len) ; } } pub fn extend_from_within_copy < R > (& mut self , src : R) where R : core :: ops :: RangeBounds < usize > , T : Copy , { let src = slice_range (src , .. self . len ()) ; let core :: ops :: Range { start , end } = src ; let len = end - start ; self . reserve (len) ; unsafe { let l = self . len () ; let ptr = self . as_mut_ptr () ; copy_nonoverlapping (ptr . add (start) , ptr . add (l) , len) ; self . set_len (l + len) ; } } pub fn insert_from_slice_copy (& mut self , index : usize , other : & [T]) where T : Copy , { let l = self . len () ; let len = other . len () ; assert ! (index <= l) ; self . reserve (len) ; unsafe { let base_ptr = self . as_mut_ptr () ; let ith_ptr = base_ptr . add (index) ; let shifted_ptr = base_ptr . add (index + len) ; copy (ith_ptr , shifted_ptr , l - index) ; copy_nonoverlapping (other . as_ptr () , ith_ptr , len) ; self . set_len (l + len) ; } } # [doc = " A function for creating [`SmallVec`] values out of slices"] # [doc = " for types with the [`Copy`] trait."] pub fn from_slice_copy (slice : & [T]) -> Self where T : Copy , { let src = slice . as_ptr () ; let len = slice . len () ; let mut result = Self :: with_capacity (len) ; unsafe { let dst = result . as_mut_ptr () ; copy_nonoverlapping (src , dst , len) ; result . set_len (len) ; } result } }
    };
}

impl_113!()
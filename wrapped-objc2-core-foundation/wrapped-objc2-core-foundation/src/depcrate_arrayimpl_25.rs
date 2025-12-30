// Generated macro for impl_25 (impl)
macro_rules! Depcrate_arrayimpl_25 {
() => {
// Module: crate::array
// Provides: {"impl_25"}
// Dependencies: {}
# [doc = " Various accessor methods."] impl < T : ? Sized > CFArray < T > { # [doc = " The amount of elements in the array."] # [inline] # [doc (alias = "CFArrayGetCount")] pub fn len (& self) -> usize { self . as_opaque () . count () as _ } # [doc = " Whether the array is empty or not."] # [inline] pub fn is_empty (& self) -> bool { self . len () == 0 } # [doc = " Retrieve the object at the given index."] # [doc = ""] # [doc = " Returns `None` if the index was out of bounds."] # [doc (alias = "CFArrayGetValueAtIndex")] pub fn get (& self , index : usize) -> Option < CFRetained < T > > where T : Type + Sized , { if index < self . len () { let index = index as CFIndex ; Some (unsafe { self . get_unchecked (index) } . retain ()) } else { None } } # [doc = " Convert the array to a `Vec` of the array's objects."] # [cfg (feature = "alloc")] # [doc (alias = "CFArrayGetValues")] pub fn to_vec (& self) -> Vec < CFRetained < T > > where T : Type + Sized , { let vec = unsafe { self . to_vec_unchecked () } ; vec . into_iter () . map (T :: retain) . collect () } # [doc = " Iterate over the array's elements."] # [inline] pub fn iter (& self) -> CFArrayIter < '_ , T > { CFArrayIter { array : self , index : 0 , } } }
};
}

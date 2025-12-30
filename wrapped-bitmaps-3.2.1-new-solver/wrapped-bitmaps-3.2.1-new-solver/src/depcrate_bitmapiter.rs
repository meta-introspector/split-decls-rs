// Generated macro for Iter (struct)
macro_rules! Depcrate_bitmapIter {
() => {
// Module: crate::bitmap
// Provides: {"Iter"}
// Dependencies: {}
# [doc = " An iterator over the indices in a bitmap which are `true`."] # [doc = ""] # [doc = " This yields a sequence of `usize` indices, not their contents (which are"] # [doc = " always `true` anyway, by definition)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```rust"] # [doc = " # use bitmaps::Bitmap;"] # [doc = " let mut bitmap: Bitmap<10> = Bitmap::new();"] # [doc = " bitmap.set(3, true);"] # [doc = " bitmap.set(5, true);"] # [doc = " bitmap.set(8, true);"] # [doc = " let true_indices: Vec<usize> = bitmap.into_iter().collect();"] # [doc = " assert_eq!(vec![3, 5, 8], true_indices);"] # [doc = " ```"] # [derive (Clone , Debug)] pub struct Iter < 'a , const SIZE : usize > where BitsImpl < SIZE > : Bits , { head : Option < usize > , tail : Option < usize > , data : & 'a Bitmap < { SIZE } > , }
};
}

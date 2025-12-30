// Generated macro for impl_42 (impl)
macro_rules! Depcrate_arrayvecimpl_42 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_42"}
// Dependencies: {}
impl < T , const N : usize > ArrayVec < T , N > { # [doc = " Create a new [`ArrayVec`]."] pub fn new () -> Self { Self { elements : [() ; N] . map (| _ | None) , length : 0 , } } # [doc = " Push an item into this [`ArrayVec`]."] pub fn push (& mut self , item : T) -> Result < () > { match self . length . checked_add (1) { Some (n) if n <= N => { self . elements [self . length] = Some (item) ; self . length = n ; Ok (()) } _ => Err (ErrorKind :: Overlength . into ()) , } } # [doc = " Get an element from this [`ArrayVec`]."] pub fn get (& self , index : usize) -> Option < & T > { match self . elements . get (index) { Some (Some (item)) => Some (item) , _ => None , } } # [doc = " Iterate over the elements in this [`ArrayVec`]."] pub fn iter (& self) -> Iter < '_ , T > { Iter :: new (& self . elements) } # [doc = " Is this [`ArrayVec`] empty?"] pub fn is_empty (& self) -> bool { self . length == 0 } # [doc = " Get the number of elements in this [`ArrayVec`]."] pub fn len (& self) -> usize { self . length } # [doc = " Get the last item from this [`ArrayVec`]."] pub fn last (& self) -> Option < & T > { self . length . checked_sub (1) . and_then (| n | self . get (n)) } # [doc = " Extract the inner array."] pub fn into_array (self) -> [Option < T > ; N] { self . elements } }
};
}

// Generated macro for impl_43 (impl)
macro_rules! Depcrateimpl_43 {
() => {
// Module: crate
// Provides: {"impl_43"}
// Dependencies: {}
impl < 'tmp , 'a : 'tmp , 'b : 'tmp > Diff < 'a , 'b > { fn text (& self) -> Range < 'tmp > { match * self { Diff :: Equal (range , _) | Diff :: Delete (range) | Diff :: Insert (range) => range , } } fn grow_left (& mut self , increment : usize) { self . for_each (| range | { range . offset -= increment ; range . len += increment ; }) ; } fn grow_right (& mut self , increment : usize) { self . for_each (| range | range . len += increment) ; } fn shift_left (& mut self , increment : usize) { self . for_each (| range | range . offset -= increment) ; } fn shift_right (& mut self , increment : usize) { self . for_each (| range | range . offset += increment) ; } fn for_each (& mut self , f : impl Fn (& mut Range)) { match self { Diff :: Equal (range1 , range2) => { f (range1) ; f (range2) ; } Diff :: Delete (range) => f (range) , Diff :: Insert (range) => f (range) , } } }
};
}

// Generated macro for impl_61 (impl)
macro_rules! Depcrate_range_set_testsimpl_61 {
() => {
// Module: crate::range_set::tests
// Provides: {"impl_61"}
// Dependencies: {}
impl RefRangeSet { fn new (capacity : usize) -> Self { Self { data : vec ! [false ; capacity] , } } fn len (& self) -> usize { let mut last = false ; let mut count = 0 ; for v in self . data . iter () { if ! last && * v { count += 1 ; } last = * v ; } count } fn is_empty (& self) -> bool { self . len () == 0 } fn insert (& mut self , x : Range < u64 >) -> bool { let mut result = false ; assert ! (x . end <= self . data . len () as u64) ; for i in x { let i = i as usize ; if ! self . data [i] { result = true ; self . data [i] = true ; } } result } fn remove (& mut self , x : Range < u64 >) -> bool { let mut result = false ; assert ! (x . end <= self . data . len () as u64) ; for i in x { let i = i as usize ; if self . data [i] { result = true ; self . data [i] = false ; } } result } fn elts (& self) -> Vec < u64 > { self . data . iter () . enumerate () . filter_map (| (i , e) | if * e { Some (i as u64) } else { None }) . collect () } }
};
}

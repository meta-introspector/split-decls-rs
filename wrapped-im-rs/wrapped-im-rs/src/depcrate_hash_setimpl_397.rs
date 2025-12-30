// Generated macro for impl_397 (impl)
macro_rules! Depcrate_hash_setimpl_397 {
() => {
// Module: crate::hash::set
// Provides: {"impl_397"}
// Dependencies: {}
impl < A , S > HashSet < A , S > where A : Hash + Eq , S : BuildHasher , { fn test_eq (& self , other : & Self) -> bool { if self . len () != other . len () { return false ; } let mut seen = collections :: HashSet :: new () ; for value in self . iter () { if ! other . contains (value) { return false ; } seen . insert (value) ; } for value in other . iter () { if ! seen . contains (& value) { return false ; } } true } # [doc = " Test if a value is part of a set."] # [doc = ""] # [doc = " Time: O(log n)"] # [must_use] pub fn contains < BA > (& self , a : & BA) -> bool where BA : Hash + Eq + ? Sized , A : Borrow < BA > , { self . root . get (hash_key (& * self . hasher , a) , 0 , a) . is_some () } # [doc = " Test whether a set is a subset of another set, meaning that"] # [doc = " all values in our set must also be in the other set."] # [doc = ""] # [doc = " Time: O(n log n)"] # [must_use] pub fn is_subset < RS > (& self , other : RS) -> bool where RS : Borrow < Self > , { let o = other . borrow () ; self . iter () . all (| a | o . contains (a)) } # [doc = " Test whether a set is a proper subset of another set, meaning"] # [doc = " that all values in our set must also be in the other set. A"] # [doc = " proper subset must also be smaller than the other set."] # [doc = ""] # [doc = " Time: O(n log n)"] # [must_use] pub fn is_proper_subset < RS > (& self , other : RS) -> bool where RS : Borrow < Self > , { self . len () != other . borrow () . len () && self . is_subset (other) } }
};
}

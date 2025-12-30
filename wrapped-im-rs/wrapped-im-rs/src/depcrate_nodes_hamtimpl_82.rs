// Generated macro for impl_82 (impl)
macro_rules! Depcrate_nodes_hamtimpl_82 {
() => {
// Module: crate::nodes::hamt
// Provides: {"impl_82"}
// Dependencies: {}
impl < A : HashValue > CollisionNode < A > { fn new (hash : HashBits , value1 : A , value2 : A) -> Self { CollisionNode { hash , data : vec ! [value1 , value2] , } } # [inline] fn len (& self) -> usize { self . data . len () } fn get < BK > (& self , key : & BK) -> Option < & A > where BK : Eq + ? Sized , A :: Key : Borrow < BK > , { for entry in & self . data { if key == entry . extract_key () . borrow () { return Some (entry) ; } } None } fn get_mut < BK > (& mut self , key : & BK) -> Option < & mut A > where BK : Eq + ? Sized , A :: Key : Borrow < BK > , { for entry in & mut self . data { if key == entry . extract_key () . borrow () { return Some (entry) ; } } None } fn insert (& mut self , value : A) -> Option < A > { for item in & mut self . data { if value . extract_key () == item . extract_key () { return Some (mem :: replace (item , value)) ; } } self . data . push (value) ; None } fn remove < BK > (& mut self , key : & BK) -> Option < A > where BK : Eq + ? Sized , A :: Key : Borrow < BK > , { let mut loc = None ; for (index , item) in self . data . iter () . enumerate () { if key == item . extract_key () . borrow () { loc = Some (index) ; } } if let Some (index) = loc { Some (self . data . remove (index)) } else { None } } fn pop (& mut self) -> Entry < A > { Entry :: Value (self . data . pop () . unwrap () , self . hash) } }
};
}

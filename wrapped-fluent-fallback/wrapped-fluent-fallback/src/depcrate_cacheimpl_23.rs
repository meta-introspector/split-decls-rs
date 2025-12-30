// Generated macro for impl_23 (impl)
macro_rules! Depcrate_cacheimpl_23 {
() => {
// Module: crate::cache
// Provides: {"impl_23"}
// Dependencies: {}
impl < I , R > Cache < I , R > where I : Iterator , { pub fn new (iter : I) -> Self { Self { iter : RefCell :: new (iter) , items : Default :: default () , res : std :: marker :: PhantomData , } } pub fn len (& self) -> usize { unsafe { let items = self . items . get () ; (* items) . len () } } pub fn get (& self , index : usize) -> Option < & I :: Item > { unsafe { let items = self . items . get () ; (* items) . get (index) } } # [doc = " Push, immediately getting a reference to the element"] pub fn push_get (& self , new_value : I :: Item) -> & I :: Item { unsafe { let items = self . items . get () ; (* items) . push_get (new_value) } } }
};
}

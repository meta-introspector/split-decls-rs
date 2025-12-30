// Generated macro for impl_29 (impl)
macro_rules! Depcrate_cacheimpl_29 {
() => {
// Module: crate::cache
// Provides: {"impl_29"}
// Dependencies: {}
impl < S , R > AsyncCache < S , R > where S : Stream , { pub fn new (stream : S) -> Self { Self { stream : PinCell :: new (stream) , items : Default :: default () , pending_wakes : Default :: default () , res : std :: marker :: PhantomData , } } pub fn len (& self) -> usize { unsafe { let items = self . items . get () ; (* items) . len () } } pub fn get (& self , index : usize) -> Poll < Option < & S :: Item > > { unsafe { let items = self . items . get () ; (* items) . get (index) . into () } } # [doc = " Push, immediately getting a reference to the element"] pub fn push_get (& self , new_value : S :: Item) -> & S :: Item { unsafe { let items = self . items . get () ; (* items) . push_get (new_value) } } pub fn stream (& self) -> AsyncCacheStream < '_ , S , R > { AsyncCacheStream { cache : self , curr : 0 , } } }
};
}

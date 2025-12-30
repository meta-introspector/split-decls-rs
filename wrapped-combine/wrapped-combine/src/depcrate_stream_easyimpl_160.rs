// Generated macro for impl_160 (impl)
macro_rules! Depcrate_stream_easyimpl_160 {
() => {
// Module: crate::stream::easy
// Provides: {"impl_160"}
// Dependencies: {}
impl < T : PartialEq , R : PartialEq > PartialEq for Info < T , R > { fn eq (& self , other : & Info < T , R >) -> bool { match (self , other) { (& Info :: Token (ref l) , & Info :: Token (ref r)) => l == r , (& Info :: Range (ref l) , & Info :: Range (ref r)) => l == r , (& Info :: Owned (ref l) , & Info :: Owned (ref r)) => l == r , (& Info :: Static (l) , & Info :: Owned (ref r)) => l == r , (& Info :: Owned (ref l) , & Info :: Static (r)) => l == r , (& Info :: Static (l) , & Info :: Static (r)) => l == r , _ => false , } } }
};
}

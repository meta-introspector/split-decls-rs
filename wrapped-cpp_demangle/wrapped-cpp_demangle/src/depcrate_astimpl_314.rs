// Generated macro for impl_314 (impl)
macro_rules! Depcrate_astimpl_314 {
() => {
// Module: crate::ast
// Provides: {"impl_314"}
// Dependencies: {}
impl < 'subs , W > DemangleAsLeaf < 'subs , W > for WellKnownComponent where W : 'subs + DemangleWrite , { fn demangle_as_leaf < 'me , 'ctx > (& 'me self , ctx : & 'ctx mut DemangleContext < 'subs , W > ,) -> fmt :: Result { match * self { WellKnownComponent :: Std => { panic ! ("should never treat `WellKnownComponent::Std` as a leaf name") } WellKnownComponent :: StdAllocator => write ! (ctx , "allocator") , WellKnownComponent :: StdString1 => write ! (ctx , "basic_string") , WellKnownComponent :: StdString2 => write ! (ctx , "string") , WellKnownComponent :: StdIstream => write ! (ctx , "basic_istream") , WellKnownComponent :: StdOstream => write ! (ctx , "ostream") , WellKnownComponent :: StdIostream => write ! (ctx , "basic_iostream") , } } }
};
}

// Generated macro for impl_1207 (impl)
macro_rules! Depcrate_runtime_nsproxyimpl_1207 {
() => {
// Module: crate::runtime::nsproxy
// Provides: {"impl_1207"}
// Dependencies: {}
unsafe impl ClassType for NSProxy { type Super = AnyObject ; type ThreadKind = dyn AnyThread ; const NAME : & 'static str = "NSProxy" ; # [inline] fn class () -> & 'static AnyClass { crate :: __class_inner ! ("NSProxy" , "NSProxy") } # [inline] fn as_super (& self) -> & Self :: Super { & self . __superclass } const __INNER : () = () ; type __SubclassingType = Self ; }
};
}

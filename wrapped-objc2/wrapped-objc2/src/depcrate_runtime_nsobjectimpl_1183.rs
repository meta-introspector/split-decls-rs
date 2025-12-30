// Generated macro for impl_1183 (impl)
macro_rules! Depcrate_runtime_nsobjectimpl_1183 {
() => {
// Module: crate::runtime::nsobject
// Provides: {"impl_1183"}
// Dependencies: {}
unsafe impl ClassType for NSObject { type Super = AnyObject ; type ThreadKind = dyn AnyThread ; const NAME : & 'static str = "NSObject" ; # [inline] fn class () -> & 'static AnyClass { crate :: __class_inner ! ("NSObject" , "NSObject") } # [inline] fn as_super (& self) -> & Self :: Super { & self . __superclass } const __INNER : () = () ; type __SubclassingType = private :: ForDefinedSubclasses ; }
};
}

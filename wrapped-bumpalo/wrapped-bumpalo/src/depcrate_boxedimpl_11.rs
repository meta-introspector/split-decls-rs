// Generated macro for impl_11 (impl)
macro_rules! Depcrate_boxedimpl_11 {
() => {
// Module: crate::boxed
// Provides: {"impl_11"}
// Dependencies: {}
impl < 'a , 'b , T : ? Sized + PartialOrd > PartialOrd < Box < 'b , T > > for Box < 'a , T > { # [inline] fn partial_cmp (& self , other : & Box < 'b , T >) -> Option < Ordering > { PartialOrd :: partial_cmp (& * * self , & * * other) } # [inline] fn lt (& self , other : & Box < 'b , T >) -> bool { PartialOrd :: lt (& * * self , & * * other) } # [inline] fn le (& self , other : & Box < 'b , T >) -> bool { PartialOrd :: le (& * * self , & * * other) } # [inline] fn ge (& self , other : & Box < 'b , T >) -> bool { PartialOrd :: ge (& * * self , & * * other) } # [inline] fn gt (& self , other : & Box < 'b , T >) -> bool { PartialOrd :: gt (& * * self , & * * other) } }
};
}

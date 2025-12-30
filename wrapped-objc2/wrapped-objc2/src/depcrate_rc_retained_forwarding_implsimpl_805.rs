// Generated macro for impl_805 (impl)
macro_rules! Depcrate_rc_retained_forwarding_implsimpl_805 {
() => {
// Module: crate::rc::retained_forwarding_impls
// Provides: {"impl_805"}
// Dependencies: {}
impl < T : ? Sized + PartialOrd < U > , U : ? Sized > PartialOrd < Retained < U > > for Retained < T > { # [inline] fn partial_cmp (& self , other : & Retained < U >) -> Option < Ordering > { (* * self) . partial_cmp (& * * other) } # [inline] fn lt (& self , other : & Retained < U >) -> bool { (* * self) . lt (& * * other) } # [inline] fn le (& self , other : & Retained < U >) -> bool { (* * self) . le (& * * other) } # [inline] fn ge (& self , other : & Retained < U >) -> bool { (* * self) . ge (& * * other) } # [inline] fn gt (& self , other : & Retained < U >) -> bool { (* * self) . gt (& * * other) } }
};
}

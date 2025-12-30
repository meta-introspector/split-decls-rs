// Generated macro for atomic_rmw (macro)
macro_rules! Depcrate_arm_linuxatomic_rmw {
() => {
// Module: crate::arm_linux
// Provides: {"atomic_rmw"}
// Dependencies: {}
macro_rules ! atomic_rmw { ($ name : ident , $ ty : ty , $ op : expr , $ fetch : expr) => { intrinsics ! { pub unsafe extern "C" fn $ name (ptr : * mut $ ty , val : $ ty) -> $ ty { unsafe { atomic_rmw (ptr , | x | $ op (x as $ ty , val) as u32 , | old , new | $ fetch (old , new)) as $ ty } } } } ; (@ old $ name : ident , $ ty : ty , $ op : expr) => { atomic_rmw ! ($ name , $ ty , $ op , | old , _ | old) ; } ; (@ new $ name : ident , $ ty : ty , $ op : expr) => { atomic_rmw ! ($ name , $ ty , $ op , | _ , new | new) ; } ; }
};
}

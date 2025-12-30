// Generated macro for copy_intrinsic (function)
macro_rules! Depcrate_mir_intrinsiccopy_intrinsic {
() => {
// Module: crate::mir::intrinsic
// Provides: {"copy_intrinsic"}
// Dependencies: {}
fn copy_intrinsic < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , allow_overlap : bool , volatile : bool , ty : Ty < 'tcx > , dst : Bx :: Value , src : Bx :: Value , count : Bx :: Value ,) { let layout = bx . layout_of (ty) ; let size = layout . size ; let align = layout . align . abi ; let size = bx . mul (bx . const_usize (size . bytes ()) , count) ; let flags = if volatile { MemFlags :: VOLATILE } else { MemFlags :: empty () } ; if allow_overlap { bx . memmove (dst , align , src , align , size , flags) ; } else { bx . memcpy (dst , align , src , align , size , flags) ; } }
};
}

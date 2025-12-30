// Generated macro for callback (macro)
macro_rules! Depcrate_machinecallback {
() => {
// Module: crate::machine
// Provides: {"callback"}
// Dependencies: {}
# [doc = " Creates a `DynMachineCallback`:"] # [doc = ""] # [doc = " ```rust"] # [doc = " callback!("] # [doc = "     @capture<'tcx> {"] # [doc = "         var1: Ty1,"] # [doc = "         var2: Ty2<'tcx>,"] # [doc = "     }"] # [doc = "     |this, arg: ArgTy| {"] # [doc = "         // Implement the callback here."] # [doc = "         todo!()"] # [doc = "     }"] # [doc = " )"] # [doc = " ```"] # [doc = ""] # [doc = " All the argument types must implement `VisitProvenance`."] # [macro_export] macro_rules ! callback { (@ capture <$ tcx : lifetime $ (,) ? $ ($ lft : lifetime) ,*> { $ ($ name : ident : $ type : ty) ,* $ (,) ? } |$ this : ident , $ arg : ident : $ arg_ty : ty | $ body : expr $ (,) ?) => { { struct Callback <$ tcx , $ ($ lft) ,*> { $ ($ name : $ type ,) * _phantom : std :: marker :: PhantomData <&$ tcx () >, } impl <$ tcx , $ ($ lft) ,*> VisitProvenance for Callback <$ tcx , $ ($ lft) ,*> { fn visit_provenance (& self , _visit : & mut VisitWith <'_ >) { $ (self .$ name . visit_provenance (_visit) ;) * } } impl <$ tcx , $ ($ lft) ,*> MachineCallback <$ tcx , $ arg_ty > for Callback <$ tcx , $ ($ lft) ,*> { fn call (self : Box < Self >, $ this : & mut MiriInterpCx <$ tcx >, $ arg : $ arg_ty) -> InterpResult <$ tcx > { # [allow (unused_variables)] let Callback { $ ($ name ,) * _phantom } = * self ; $ body } } Box :: new (Callback { $ ($ name ,) * _phantom : std :: marker :: PhantomData }) } } ; }
};
}

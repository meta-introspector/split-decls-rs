// Generated macro for method_to_assoc_fn_call_self_adjust (function)
macro_rules! Depcrate_renamemethod_to_assoc_fn_call_self_adjust {
() => {
// Module: crate::rename
// Provides: {"method_to_assoc_fn_call_self_adjust"}
// Dependencies: {}
fn method_to_assoc_fn_call_self_adjust (sema : & Semantics < '_ , RootDatabase > , self_arg : & ast :: Expr ,) -> CallReceiverAdjust { let mut result = CallReceiverAdjust :: None ; let self_adjust = sema . expr_adjustments (self_arg) ; if let Some (self_adjust) = self_adjust { let mut i = 0 ; while i < self_adjust . len () { if matches ! (self_adjust [i] . kind , hir :: Adjust :: Deref (..)) && matches ! (self_adjust . get (i + 1) , Some (hir :: Adjustment { kind : hir :: Adjust :: Borrow (..) , .. })) { i += 2 ; continue ; } match self_adjust [i] . kind { hir :: Adjust :: Deref (_) if result == CallReceiverAdjust :: None => { result = CallReceiverAdjust :: Deref ; } hir :: Adjust :: Borrow (hir :: AutoBorrow :: Ref (mutability)) => { match (result , mutability) { (CallReceiverAdjust :: RefMut , hir :: Mutability :: Shared) => { } (_ , hir :: Mutability :: Mut) => result = CallReceiverAdjust :: RefMut , (_ , hir :: Mutability :: Shared) => result = CallReceiverAdjust :: Ref , } } _ => { } } i += 1 ; } } result }
};
}

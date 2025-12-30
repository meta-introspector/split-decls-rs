// Generated macro for mir_is_empty (function)
macro_rules! Depcrate_constsmir_is_empty {
() => {
// Module: crate::consts
// Provides: {"mir_is_empty"}
// Dependencies: {}
fn mir_is_empty < 'tcx > (tcx : TyCtxt < 'tcx > , result : mir :: Const < 'tcx >) -> Option < bool > { let mir :: Const :: Val (val , _) = result else { return None ; } ; match (val , result . ty () . kind ()) { (_ , ty :: Ref (_ , inner_ty , _)) => match inner_ty . kind () { ty :: Str | ty :: Slice (_) => { if let ConstValue :: Indirect { alloc_id , offset } = val { let a = tcx . global_alloc (alloc_id) . unwrap_memory () . inner () ; let ptr_size = tcx . data_layout . pointer_size () ; if a . size () < offset + 2 * ptr_size { return None ; } let len = a . read_scalar (& tcx , alloc_range (offset + ptr_size , ptr_size) , false) . ok () ? . to_target_usize (& tcx) . discard_err () ? ; Some (len == 0) } else { None } } , ty :: Array (_ , len) => Some (len . try_to_target_usize (tcx) ? == 0) , _ => None , } , (ConstValue :: Indirect { .. } , ty :: Array (_ , len)) => Some (len . try_to_target_usize (tcx) ? == 0) , (ConstValue :: ZeroSized , _) => Some (true) , _ => None , } }
};
}

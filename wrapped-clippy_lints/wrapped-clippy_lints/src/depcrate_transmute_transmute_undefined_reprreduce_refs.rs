// Generated macro for reduce_refs (function)
macro_rules! Depcrate_transmute_transmute_undefined_reprreduce_refs {
() => {
// Module: crate::transmute::transmute_undefined_repr
// Provides: {"reduce_refs"}
// Dependencies: {}
# [doc = " Remove references so long as both types are references."] fn reduce_refs < 'tcx > (cx : & LateContext < 'tcx > , mut from_ty : Ty < 'tcx > , mut to_ty : Ty < 'tcx >) -> ReducedTys < 'tcx > { let mut from_raw_ptr = false ; let mut to_raw_ptr = false ; let (from_fat_ptr , to_fat_ptr) = loop { break match (from_ty . kind () , to_ty . kind ()) { (& (ty :: Ref (_ , from_sub_ty , _) | ty :: RawPtr (from_sub_ty , _)) , & (ty :: Ref (_ , to_sub_ty , _) | ty :: RawPtr (to_sub_ty , _)) ,) => { from_raw_ptr = matches ! (* from_ty . kind () , ty :: RawPtr (_ , _)) ; from_ty = from_sub_ty ; to_raw_ptr = matches ! (* to_ty . kind () , ty :: RawPtr (_ , _)) ; to_ty = to_sub_ty ; continue ; } , (& (ty :: Ref (_ , unsized_ty , _) | ty :: RawPtr (unsized_ty , _)) , _) if ! unsized_ty . is_sized (cx . tcx , cx . typing_env ()) => { (true , false) } , (_ , & (ty :: Ref (_ , unsized_ty , _) | ty :: RawPtr (unsized_ty , _))) if ! unsized_ty . is_sized (cx . tcx , cx . typing_env ()) => { (false , true) } , _ => (false , false) , } ; } ; ReducedTys { from_ty , to_ty , from_raw_ptr , to_raw_ptr , from_fat_ptr , to_fat_ptr , } }
};
}

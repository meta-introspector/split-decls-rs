// Generated macro for unsize_ptr (function)
macro_rules! Depcrate_unsizeunsize_ptr {
() => {
// Module: crate::unsize
// Provides: {"unsize_ptr"}
// Dependencies: {}
# [doc = " Coerce `src` to `dst_ty`."] fn unsize_ptr < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , src : Value , src_layout : TyAndLayout < 'tcx > , dst_layout : TyAndLayout < 'tcx > , old_info : Option < Value > ,) -> (Value , Value) { match (& src_layout . ty . kind () , & dst_layout . ty . kind ()) { (& ty :: Ref (_ , a , _) , & ty :: Ref (_ , b , _)) | (& ty :: Ref (_ , a , _) , & ty :: RawPtr (b , _)) | (& ty :: RawPtr (a , _) , & ty :: RawPtr (b , _)) => (src , unsized_info (fx , * a , * b , old_info)) , (& ty :: Adt (def_a , _) , & ty :: Adt (def_b , _)) => { assert_eq ! (def_a , def_b) ; if src_layout == dst_layout { return (src , old_info . unwrap ()) ; } let mut result = None ; for i in 0 .. src_layout . fields . count () { let src_f = src_layout . field (fx , i) ; assert_eq ! (src_layout . fields . offset (i) . bytes () , 0) ; assert_eq ! (dst_layout . fields . offset (i) . bytes () , 0) ; if src_f . is_1zst () { continue ; } assert_eq ! (src_layout . size , src_f . size) ; let dst_f = dst_layout . field (fx , i) ; assert_ne ! (src_f . ty , dst_f . ty) ; assert_eq ! (result , None) ; result = Some (unsize_ptr (fx , src , src_f , dst_f , old_info)) ; } result . unwrap () } _ => bug ! ("unsize_ptr: called on bad types") , } }
};
}

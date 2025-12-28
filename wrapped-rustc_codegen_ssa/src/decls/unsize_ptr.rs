macro_rules! deps {
    () => {
        BuilderMethods!();
    };
}

macro_rules! unsize_ptr {
    () => {
        deps!();
        # [doc = " Coerces `src` to `dst_ty`. `src_ty` must be a pointer."] pub (crate) fn unsize_ptr < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , src : Bx :: Value , src_ty : Ty < 'tcx > , dst_ty : Ty < 'tcx > , old_info : Option < Bx :: Value > ,) -> (Bx :: Value , Bx :: Value) { debug ! ("unsize_ptr: {:?} => {:?}" , src_ty , dst_ty) ; match (src_ty . kind () , dst_ty . kind ()) { (& ty :: Ref (_ , a , _) , & ty :: Ref (_ , b , _) | & ty :: RawPtr (b , _)) | (& ty :: RawPtr (a , _) , & ty :: RawPtr (b , _)) => { assert_eq ! (bx . cx () . type_is_sized (a) , old_info . is_none ()) ; (src , unsized_info (bx , a , b , old_info)) } (& ty :: Adt (def_a , _) , & ty :: Adt (def_b , _)) => { assert_eq ! (def_a , def_b) ; let src_layout = bx . cx () . layout_of (src_ty) ; let dst_layout = bx . cx () . layout_of (dst_ty) ; if src_ty == dst_ty { return (src , old_info . unwrap ()) ; } let mut result = None ; for i in 0 .. src_layout . fields . count () { let src_f = src_layout . field (bx . cx () , i) ; if src_f . is_1zst () { continue ; } assert_eq ! (src_layout . fields . offset (i) . bytes () , 0) ; assert_eq ! (dst_layout . fields . offset (i) . bytes () , 0) ; assert_eq ! (src_layout . size , src_f . size) ; let dst_f = dst_layout . field (bx . cx () , i) ; assert_ne ! (src_f . ty , dst_f . ty) ; assert_eq ! (result , None) ; result = Some (unsize_ptr (bx , src , src_f . ty , dst_f . ty , old_info)) ; } result . unwrap () } _ => bug ! ("unsize_ptr: called on bad types") , } }
    };
}

unsize_ptr!()
macro_rules! add_trait_bound {
    () => {
        # [doc = " Add a trait bound for `did`."] fn add_trait_bound < 'tcx > (tcx : TyCtxt < 'tcx > , bounds : & mut Vec < (ty :: Clause < 'tcx > , Span) > , self_ty : Ty < 'tcx > , did : DefId , span : Span ,) { let trait_ref = ty :: TraitRef :: new (tcx , did , [self_ty]) ; bounds . insert (0 , (trait_ref . upcast (tcx) , span)) ; }
    };
}

add_trait_bound!();
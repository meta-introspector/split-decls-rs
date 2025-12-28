macro_rules! deps {
    () => {
        TypeFreshener!();
        InferCtxt!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        impl < 'a , 'tcx > TypeFreshener < 'a , 'tcx > { pub fn new (infcx : & 'a InferCtxt < 'tcx >) -> TypeFreshener < 'a , 'tcx > { TypeFreshener { infcx , ty_freshen_count : 0 , const_freshen_count : 0 , ty_freshen_map : Default :: default () , const_freshen_map : Default :: default () , } } fn freshen_ty < F > (& mut self , input : Result < Ty < 'tcx > , ty :: InferTy > , mk_fresh : F) -> Ty < 'tcx > where F : FnOnce (u32) -> Ty < 'tcx > , { match input { Ok (ty) => ty . fold_with (self) , Err (key) => match self . ty_freshen_map . entry (key) { Entry :: Occupied (entry) => * entry . get () , Entry :: Vacant (entry) => { let index = self . ty_freshen_count ; self . ty_freshen_count += 1 ; let t = mk_fresh (index) ; entry . insert (t) ; t } } , } } fn freshen_const < F > (& mut self , input : Result < ty :: Const < 'tcx > , ty :: InferConst > , freshener : F ,) -> ty :: Const < 'tcx > where F : FnOnce (u32) -> ty :: InferConst , { match input { Ok (ct) => ct . fold_with (self) , Err (key) => match self . const_freshen_map . entry (key) { Entry :: Occupied (entry) => * entry . get () , Entry :: Vacant (entry) => { let index = self . const_freshen_count ; self . const_freshen_count += 1 ; let ct = ty :: Const :: new_infer (self . infcx . tcx , freshener (index)) ; entry . insert (ct) ; ct } } , } } }
    };
}

impl_56!();
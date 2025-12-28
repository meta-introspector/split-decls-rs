macro_rules! deps {
    () => {
        InferCtxt!();
    };
}

macro_rules! TypeFreshener {
    () => {
        deps!();
        pub struct TypeFreshener < 'a , 'tcx > { infcx : & 'a InferCtxt < 'tcx > , ty_freshen_count : u32 , const_freshen_count : u32 , ty_freshen_map : FxHashMap < ty :: InferTy , Ty < 'tcx > > , const_freshen_map : FxHashMap < ty :: InferConst , ty :: Const < 'tcx > > , }
    };
}

TypeFreshener!()
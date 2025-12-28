macro_rules! deps {
    () => {
        TraitEngine!();
        InferOk!();
        InferCtxt!();
    };
}

macro_rules! impl_269 {
    () => {
        deps!();
        impl < 'tcx , T > InferOk < 'tcx , T > { # [doc = " Extracts `value`, registering any obligations into `fulfill_cx`."] pub fn into_value_registering_obligations < E : 'tcx > (self , infcx : & InferCtxt < 'tcx > , fulfill_cx : & mut dyn TraitEngine < 'tcx , E > ,) -> T { let InferOk { value , obligations } = self ; fulfill_cx . register_predicate_obligations (infcx , obligations) ; value } }
    };
}

impl_269!()
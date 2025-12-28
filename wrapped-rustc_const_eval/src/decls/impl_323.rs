macro_rules! deps {
    () => {
        FrameInfo!();
        FrameNote!();
    };
}

macro_rules! impl_323 {
    () => {
        deps!();
        impl < 'tcx > FrameInfo < 'tcx > { pub fn as_note (& self , tcx : TyCtxt < 'tcx >) -> errors :: FrameNote { let span = self . span ; if tcx . def_key (self . instance . def_id ()) . disambiguated_data . data == DefPathData :: Closure { errors :: FrameNote { where_ : "closure" , span , instance : String :: new () , times : 0 , has_label : false , } } else { let instance = format ! ("{}" , self . instance) ; errors :: FrameNote { where_ : "instance" , span , instance , times : 0 , has_label : false } } } }
    };
}

impl_323!();
macro_rules! deps {
    () => {
        FrameInfo!();
    };
}

macro_rules! impl_322 {
    () => {
        deps!();
        impl < 'tcx > fmt :: Display for FrameInfo < 'tcx > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { ty :: tls :: with (| tcx | { if tcx . def_key (self . instance . def_id ()) . disambiguated_data . data == DefPathData :: Closure { write ! (f , "inside closure") } else { write ! (f , "inside `{}`" , self . instance) } }) } }
    };
}

impl_322!()
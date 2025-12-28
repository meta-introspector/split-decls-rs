macro_rules! deps {
    () => {
        ReturnDefault!();
        DefaultReturner!();
    };
}

macro_rules! macro_11 {
    () => {
        deps!();
        :: cfg_if :: cfg_if ! { if # [cfg (feature = "nightly")] { impl < O > ReturnDefault < O > for DefaultReturner < O > { default fn maybe_return_default () -> Option < O > { None } default fn return_default () -> Result < O , &'static str > { Err ("Can only return default values for types that impl std::Default") } } impl < O : Default > ReturnDefault < O > for DefaultReturner < O > { fn maybe_return_default () -> Option < O > { Some (O :: default ()) } fn return_default () -> Result < O , &'static str > { Ok (O :: default ()) } } } else { impl < O > ReturnDefault < O > for DefaultReturner < O > { fn maybe_return_default () -> Option < O > { None } fn return_default () -> Result < O , &'static str > { Err ("Returning default values requires the \"nightly\" feature") } } } }
    };
}

macro_11!()
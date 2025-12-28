macro_rules! NullFormatter {
    () => {
        # [derive (Copy , Clone , Default , Debug)] struct NullFormatter (& 'static str) ;
    };
}

NullFormatter!()
macro_rules! deps {
    () => {
        CodegenCx!();
    };
}

macro_rules! DIB {
    () => {
        deps!();
        # [inline] # [allow (non_snake_case)] pub (crate) fn DIB < 'a , 'll > (cx : & 'a CodegenCx < 'll , '_ >) -> & 'a DIBuilder < 'll > { cx . dbg_cx . as_ref () . unwrap () . builder . as_ref () }
    };
}

DIB!();
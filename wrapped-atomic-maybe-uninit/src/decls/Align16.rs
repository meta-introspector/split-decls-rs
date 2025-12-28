macro_rules! Align16 {
    () => {
        # [repr (C , align (16))] pub (crate) struct Align16 < T > (pub (crate) T) ;
    };
}

Align16!()
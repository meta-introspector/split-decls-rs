macro_rules! ChainFn {
    () => {
        # [derive (Debug , Copy , Clone , Default)] pub struct ChainFn < F , G > (F , G) ;
    };
}

ChainFn!();
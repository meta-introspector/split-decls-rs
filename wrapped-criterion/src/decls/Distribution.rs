macro_rules! Distribution {
    () => {
        # [doc = " The bootstrap distribution of some parameter"] # [derive (Clone)] pub struct Distribution < A > (Box < [A] >) ;
    };
}

Distribution!();
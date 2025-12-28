macro_rules! N {
    () => {
        # [cfg (feature = "arbitrary_precision")] type N = String ;
    };
}

N!()
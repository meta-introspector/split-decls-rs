macro_rules! deps {
    () => {
        Canceled!();
    };
}

macro_rules! impl_116 {
    () => {
        deps!();
        # [cfg (feature = "std")] impl std :: error :: Error for Canceled { }
    };
}

impl_116!()
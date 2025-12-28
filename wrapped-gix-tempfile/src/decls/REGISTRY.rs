macro_rules! deps {
    () => {
        ForksafeTempfile!();
        HashMap!();
    };
}

macro_rules! REGISTRY {
    () => {
        deps!();
        static REGISTRY : LazyLock < HashMap < usize , Option < ForksafeTempfile > > > = LazyLock :: new (HashMap :: default) ;
    };
}

REGISTRY!()
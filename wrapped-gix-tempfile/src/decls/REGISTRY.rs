macro_rules! deps {
    () => {
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
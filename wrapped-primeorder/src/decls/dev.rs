macro_rules! dev {
    () => {
        # [cfg (feature = "dev")] mod dev ;
    };
}

dev!()
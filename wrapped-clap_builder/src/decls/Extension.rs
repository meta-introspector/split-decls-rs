macro_rules! Extension {
    () => {
        # [allow (unreachable_pub)] pub trait Extension : std :: fmt :: Debug + Clone + std :: any :: Any + Send + Sync + 'static { }
    };
}

Extension!()
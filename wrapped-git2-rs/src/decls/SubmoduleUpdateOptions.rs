macro_rules! deps {
    () => {
        FetchOptions!();
        CheckoutBuilder!();
    };
}

macro_rules! SubmoduleUpdateOptions {
    () => {
        deps!();
        # [doc = " Options to update a submodule."] pub struct SubmoduleUpdateOptions < 'cb > { checkout_builder : CheckoutBuilder < 'cb > , fetch_opts : FetchOptions < 'cb > , allow_fetch : bool , }
    };
}

SubmoduleUpdateOptions!();
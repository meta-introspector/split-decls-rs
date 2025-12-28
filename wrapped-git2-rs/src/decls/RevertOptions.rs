macro_rules! deps {
    () => {
        CheckoutBuilder!();
        MergeOptions!();
    };
}

macro_rules! RevertOptions {
    () => {
        deps!();
        # [doc = " Options to specify when reverting"] pub struct RevertOptions < 'cb > { mainline : u32 , checkout_builder : Option < CheckoutBuilder < 'cb > > , merge_opts : Option < MergeOptions > , }
    };
}

RevertOptions!();
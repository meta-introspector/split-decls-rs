macro_rules! deps {
    () => {
        MergeOptions!();
        CheckoutBuilder!();
    };
}

macro_rules! CherrypickOptions {
    () => {
        deps!();
        # [doc = " Options to specify when cherry picking"] pub struct CherrypickOptions < 'cb > { mainline : u32 , checkout_builder : Option < CheckoutBuilder < 'cb > > , merge_opts : Option < MergeOptions > , }
    };
}

CherrypickOptions!();
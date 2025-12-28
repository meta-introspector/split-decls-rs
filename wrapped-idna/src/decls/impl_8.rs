macro_rules! deps {
    () => {
        Config!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [doc = " The defaults are that of _beStrict=false_ in the [WHATWG URL Standard](https://url.spec.whatwg.org/#idna)"] impl Default for Config { fn default () -> Self { Self { use_std3_ascii_rules : false , transitional_processing : false , check_hyphens : false , verify_dns_length : false , } } }
    };
}

impl_8!();
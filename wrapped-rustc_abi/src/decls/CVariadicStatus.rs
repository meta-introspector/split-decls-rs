macro_rules! CVariadicStatus {
    () => {
        # [cfg (feature = "nightly")] pub enum CVariadicStatus { NotSupported , Stable , Unstable { feature : Symbol } , }
    };
}

CVariadicStatus!();
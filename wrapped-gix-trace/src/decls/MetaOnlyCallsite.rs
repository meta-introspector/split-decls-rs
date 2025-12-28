macro_rules! MetaOnlyCallsite {
    () => {
        # [doc (hidden)] pub struct MetaOnlyCallsite (pub & 'static Metadata < 'static >) ;
    };
}

MetaOnlyCallsite!();
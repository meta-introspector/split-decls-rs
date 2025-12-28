macro_rules! HirPlaceholderCollector {
    () => {
        # [derive (Default)] pub (crate) struct HirPlaceholderCollector { pub spans : Vec < Span > , pub may_contain_const_infer : bool , }
    };
}

HirPlaceholderCollector!();
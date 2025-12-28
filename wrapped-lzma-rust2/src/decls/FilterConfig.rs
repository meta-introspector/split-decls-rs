macro_rules! deps {
    () => {
        FilterType!();
    };
}

macro_rules! FilterConfig {
    () => {
        deps!();
        # [doc = " Configuration for a filter in the XZ filter chain."] # [derive (Debug , Clone)] pub struct FilterConfig { # [doc = " Filter type to use."] pub filter_type : FilterType , # [doc = " Property to use."] pub property : u32 , }
    };
}

FilterConfig!();
// Generated macro for Standard (struct)
macro_rules! Depcrate_standardStandard {
() => {
// Module: crate::standard
// Provides: {"Standard"}
// Dependencies: {}
# [doc = " The standard printer, which implements grep-like formatting, including"] # [doc = " color support."] # [doc = ""] # [doc = " A default printer can be created with either of the `Standard::new` or"] # [doc = " `Standard::new_no_color` constructors. However, there are a considerable"] # [doc = " number of options that configure this printer's output. Those options can"] # [doc = " be configured using [`StandardBuilder`]."] # [doc = ""] # [doc = " This type is generic over `W`, which represents any implementation"] # [doc = " of the `termcolor::WriteColor` trait. If colors are not desired,"] # [doc = " then the `new_no_color` constructor can be used, or, alternatively,"] # [doc = " the `termcolor::NoColor` adapter can be used to wrap any `io::Write`"] # [doc = " implementation without enabling any colors."] # [derive (Clone , Debug)] pub struct Standard < W > { config : Config , wtr : RefCell < CounterWriter < W > > , matches : Vec < Match > , }
};
}

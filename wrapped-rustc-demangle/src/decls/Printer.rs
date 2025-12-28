macro_rules! deps {
    () => {
        Parser!();
        ParseError!();
    };
}

macro_rules! Printer {
    () => {
        deps!();
        struct Printer < 'a , 'b : 'a , 's > { # [doc = " The input parser to demangle from, or `Err` if any (parse) error was"] # [doc = " encountered (in order to disallow further likely-incorrect demangling)."] # [doc = ""] # [doc = " See also the documentation on the `invalid!` and `parse!` macros below."] parser : Result < Parser < 's > , ParseError > , # [doc = " The output formatter to demangle to, or `None` while skipping printing."] out : Option < & 'a mut fmt :: Formatter < 'b > > , # [doc = " Cumulative number of lifetimes bound by `for<...>` binders ('G'),"] # [doc = " anywhere \"around\" the current entity (e.g. type) being demangled."] # [doc = " This value is not tracked while skipping printing, as it'd be unused."] # [doc = ""] # [doc = " See also the documentation on the `Printer::in_binder` method."] bound_lifetime_depth : u32 , }
    };
}

Printer!()
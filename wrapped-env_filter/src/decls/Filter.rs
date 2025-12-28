macro_rules! deps {
    () => {
        Builder!();
        FilterOp!();
        Directive!();
    };
}

macro_rules! Filter {
    () => {
        deps!();
        # [doc = " A log filter."] # [doc = ""] # [doc = " This struct can be used to determine whether or not a log record"] # [doc = " should be written to the output."] # [doc = " Use the [`Builder`] type to parse and construct a `Filter`."] # [doc = ""] # [doc = " [`Builder`]: struct.Builder.html"] # [derive (Clone)] pub struct Filter { directives : Vec < Directive > , filter : Option < FilterOp > , }
    };
}

Filter!()
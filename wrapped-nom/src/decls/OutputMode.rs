macro_rules! deps {
    () => {
        IsStreaming!();
        Mode!();
        Check!();
        Emit!();
        Error!();
    };
}

macro_rules! OutputMode {
    () => {
        deps!();
        # [doc = " Trait Defining the parser's execution"] # [doc = ""] # [doc = " The same parser implementation can vary in behaviour according to the chosen"] # [doc = " output mode"] pub trait OutputMode { # [doc = " Defines the [Mode] for the output type. [Emit] will generate the value, [Check] will"] # [doc = " apply the parser but will only generate `()` if successful. This can be used when"] # [doc = " verifying that the input data conforms to the format without having to generate any"] # [doc = " output data"] type Output : Mode ; # [doc = " Defines the [Mode] for the output type. [Emit] will generate the value, [Check] will"] # [doc = " apply the parser but will only generate `()` if an error happened. [Emit] should be"] # [doc = " used when we want to handle the error and extract useful information from it. [Check]"] # [doc = " is used when we just want to know if parsing failed and reject the data quickly."] type Error : Mode ; # [doc = " Indicates whether the input data is \"complete\", ie we already have the entire data in the"] # [doc = " buffer, or if it is \"streaming\", where more data can be added later in the buffer. In"] # [doc = " streaming mode, the parser will understand that a failure may mean that we are missing"] # [doc = " data, and will return a specific error branch, [Err::Incomplete] to signal it. In complete"] # [doc = " mode, the parser will generate a normal error"] type Incomplete : IsStreaming ; }
    };
}

OutputMode!()
macro_rules! deps {
    () => {
        StartBytesBuilder!();
        RareBytesBuilder!();
        MemmemBuilder!();
    };
}

macro_rules! Builder {
    () => {
        deps!();
        # [doc = " A builder for constructing the best possible prefilter. When constructed,"] # [doc = " this builder will heuristically select the best prefilter it can build,"] # [doc = " if any, and discard the rest."] # [derive (Debug)] pub (crate) struct Builder { count : usize , ascii_case_insensitive : bool , start_bytes : StartBytesBuilder , rare_bytes : RareBytesBuilder , memmem : MemmemBuilder , packed : Option < packed :: Builder > , enabled : bool , }
    };
}

Builder!()
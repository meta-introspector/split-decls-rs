macro_rules! deps {
    () => {
        ValueSource!();
        AnyValueId!();
        AnyValue!();
    };
}

macro_rules! MatchedArg {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub (crate) struct MatchedArg { source : Option < ValueSource > , indices : Vec < usize > , type_id : Option < AnyValueId > , vals : Vec < Vec < AnyValue > > , raw_vals : Vec < Vec < OsString > > , ignore_case : bool , }
    };
}

MatchedArg!();
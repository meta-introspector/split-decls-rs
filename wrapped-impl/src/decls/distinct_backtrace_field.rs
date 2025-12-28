macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! distinct_backtrace_field {
    () => {
        deps!();
        fn distinct_backtrace_field < 'a , 'b > (backtrace_field : & 'a Field < 'b > , from_field : Option < & Field > ,) -> Option < & 'a Field < 'b > > { if from_field . map_or (false , | from_field | { from_field . member == backtrace_field . member }) { None } else { Some (backtrace_field) } }
    };
}

distinct_backtrace_field!()
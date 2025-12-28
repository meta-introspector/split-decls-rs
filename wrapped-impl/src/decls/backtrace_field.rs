macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! backtrace_field {
    () => {
        deps!();
        fn backtrace_field < 'a , 'b > (fields : & 'a [Field < 'b >]) -> Option < & 'a Field < 'b > > { for field in fields { if field . attrs . backtrace . is_some () { return Some (field) ; } } for field in fields { if field . is_backtrace () { return Some (field) ; } } None }
    };
}

backtrace_field!();
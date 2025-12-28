macro_rules! deps {
    () => {
        CandidateSource!();
    };
}

macro_rules! MethodError {
    () => {
        deps!();
        # [derive (Debug)] pub enum MethodError < 'db > { # [doc = " Did not find an applicable method."] NoMatch , # [doc = " Multiple methods might apply."] Ambiguity (Vec < CandidateSource >) , # [doc = " Found an applicable method, but it is not visible."] PrivateMatch (Pick < 'db >) , # [doc = " Found a `Self: Sized` bound where `Self` is a trait object."] IllegalSizedBound { candidates : Vec < FunctionId > , needs_mut : bool } , # [doc = " Error has already been emitted, no need to emit another one."] ErrorReported , }
    };
}

MethodError!();
macro_rules! InitError {
    () => {
        # [doc = " Information about why a type cannot be initialized this way."] pub struct InitError { pub (crate) message : String , # [doc = " Spans from struct fields and similar that can be obtained from just the type."] pub (crate) span : Option < Span > , # [doc = " Used to report a trace through adts."] pub (crate) nested : Option < Box < InitError > > , }
    };
}

InitError!();
macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! FrontmatterError {
    () => {
        deps!();
        # [derive (Debug)] pub struct FrontmatterError { message : String , primary_span : Span , visible_spans : Vec < Span > , }
    };
}

FrontmatterError!();
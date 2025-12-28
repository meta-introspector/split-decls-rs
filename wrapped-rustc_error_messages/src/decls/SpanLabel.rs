macro_rules! deps {
    () => {
        DiagMessage!();
    };
}

macro_rules! SpanLabel {
    () => {
        deps!();
        # [doc = " A span together with some additional data."] # [derive (Clone , Debug)] pub struct SpanLabel { # [doc = " The span we are going to include in the final snippet."] pub span : Span , # [doc = " Is this a primary span? This is the \"locus\" of the message,"] # [doc = " and is indicated with a `^^^^` underline, versus `----`."] pub is_primary : bool , # [doc = " What label should we attach to this span (if any)?"] pub label : Option < DiagMessage > , }
    };
}

SpanLabel!();
macro_rules! Upvar {
    () => {
        # [doc = " A variable captured by a closure."] # [derive (Debug , Copy , Clone , HashStable_Generic)] pub struct Upvar { # [doc = " First span where it is accessed (there can be multiple)."] pub span : Span , }
    };
}

Upvar!();
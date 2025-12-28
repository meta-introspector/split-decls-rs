macro_rules! CrlfRoundTripCheck {
    () => {
        # [doc = " Define how to perform CRLF round-trip checking when converting to git."] # [derive (Default , Debug , Copy , Clone , Eq , PartialEq)] pub enum CrlfRoundTripCheck { # [doc = " Fail with an error if CRLF conversion isn't round-trip safe."] Fail , # [doc = " Emit a warning using `gix_trace::warn!`, but don't fail."] # [doc = ""] # [doc = " Note that the parent application has to setup tracing to make these events visible, along with a parent `span!`."] # [default] Warn , # [doc = " Do nothing, do not perform round-trip check at all."] Skip , }
    };
}

CrlfRoundTripCheck!()
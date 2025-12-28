macro_rules! ProcessingSuccess {
    () => {
        # [doc = " The success outcome of [`Uts46::process`]"] # [derive (PartialEq , Eq , Copy , Clone , Debug)] pub enum ProcessingSuccess { # [doc = " There were no errors. The caller must consider the input to be the output."] # [doc = ""] # [doc = " This asserts that the input can be safely passed to [`core::str::from_utf8_unchecked`]."] # [doc = ""] # [doc = " (Distinct from `WroteToSink` in order to allow `Cow` behavior to be implemented on top of"] # [doc = " [`Uts46::process`].)"] Passthrough , # [doc = " There were no errors. The caller must consider what was written to the sink to be the output."] # [doc = ""] # [doc = " (Distinct from `Passthrough` in order to allow `Cow` behavior to be implemented on top of"] # [doc = " [`Uts46::process`].)"] WroteToSink , }
    };
}

ProcessingSuccess!()
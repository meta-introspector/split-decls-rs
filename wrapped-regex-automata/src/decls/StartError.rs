macro_rules! deps {
    () => {
        DFA!();
        CacheError!();
        Cache!();
        Anchored!();
    };
}

macro_rules! StartError {
    () => {
        deps!();
        # [doc = " An error that can occur when computing the start state for a search."] # [doc = ""] # [doc = " Computing a start state can fail for a few reasons, either"] # [doc = " based on incorrect configuration or even based on whether"] # [doc = " the look-behind byte triggers a quit state. Typically"] # [doc = " one does not need to handle this error if you're using"] # [doc = " [`DFA::start_state_forward`](crate::hybrid::dfa::DFA::start_state_forward)"] # [doc = " (or its reverse counterpart), as that routine automatically converts"] # [doc = " `StartError` to a [`MatchError`](crate::MatchError) for you."] # [doc = ""] # [doc = " This error may be returned by the"] # [doc = " [`DFA::start_state`](crate::hybrid::dfa::DFA::start_state) routine."] # [doc = ""] # [doc = " This error implements the `std::error::Error` trait when the `std` feature"] # [doc = " is enabled."] # [doc = ""] # [doc = " This error is marked as non-exhaustive. New variants may be added in a"] # [doc = " semver compatible release."] # [non_exhaustive] # [derive (Clone , Debug)] pub enum StartError { # [doc = " An error that occurs when cache inefficiency has dropped below the"] # [doc = " configured heuristic thresholds."] Cache { # [doc = " The underlying cache error that occurred."] err : CacheError , } , # [doc = " An error that occurs when a starting configuration's look-behind byte"] # [doc = " is in this DFA's quit set."] Quit { # [doc = " The quit byte that was found."] byte : u8 , } , # [doc = " An error that occurs when the caller requests an anchored mode that"] # [doc = " isn't supported by the DFA."] UnsupportedAnchored { # [doc = " The anchored mode given that is unsupported."] mode : Anchored , } , }
    };
}

StartError!()
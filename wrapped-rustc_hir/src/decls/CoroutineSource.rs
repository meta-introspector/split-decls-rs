macro_rules! deps {
    () => {
        Block!();
        Closure!();
    };
}

macro_rules! CoroutineSource {
    () => {
        deps!();
        # [doc = " In the case of a coroutine created as part of an async/gen construct,"] # [doc = " which kind of async/gen construct caused it to be created?"] # [doc = ""] # [doc = " This helps error messages but is also used to drive coercions in"] # [doc = " type-checking (see #60424)."] # [derive (Clone , PartialEq , Eq , Hash , Debug , Copy , HashStable_Generic , Encodable , Decodable)] pub enum CoroutineSource { # [doc = " An explicit `async`/`gen` block written by the user."] Block , # [doc = " An explicit `async`/`gen` closure written by the user."] Closure , # [doc = " The `async`/`gen` block generated as the body of an async/gen function."] Fn , }
    };
}

CoroutineSource!();
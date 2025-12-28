macro_rules! FutureKind {
    () => {
        # [doc = " Used by [`LoweringContext::make_lowered_await`] to customize the desugaring based on what kind"] # [doc = " of future we are awaiting."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] enum FutureKind { # [doc = " We are awaiting a normal future"] Future , # [doc = " We are awaiting something that's known to be an AsyncIterator (i.e. we are in the header of"] # [doc = " a `for await` loop)"] AsyncIterator , }
    };
}

FutureKind!()
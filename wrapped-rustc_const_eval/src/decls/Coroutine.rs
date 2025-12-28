macro_rules! Coroutine {
    () => {
        # [derive (Debug)] pub (crate) struct Coroutine (pub hir :: CoroutineKind) ;
    };
}

Coroutine!()
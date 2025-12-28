macro_rules! LaterUseKind {
    () => {
        # [derive (Clone , Copy , Debug)] pub (crate) enum LaterUseKind { TraitCapture , ClosureCapture , Call , FakeLetRead , Other , }
    };
}

LaterUseKind!();
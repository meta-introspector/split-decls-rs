macro_rules! FrameInfo {
    () => {
        # [doc = " What we store about a frame in an interpreter backtrace."] # [derive (Clone , Debug)] pub struct FrameInfo < 'tcx > { pub instance : ty :: Instance < 'tcx > , pub span : Span , }
    };
}

FrameInfo!();
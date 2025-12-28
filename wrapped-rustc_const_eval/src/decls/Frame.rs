macro_rules! deps {
    () => {
        PlaceTy!();
        MPlaceTy!();
        LocalState!();
        SpanGuard!();
        ReturnContinuation!();
    };
}

macro_rules! Frame {
    () => {
        deps!();
        # [doc = " A stack frame."] pub struct Frame < 'tcx , Prov : Provenance = CtfeProvenance , Extra = () > { # [doc = " The MIR for the function called on this frame."] pub (super) body : & 'tcx mir :: Body < 'tcx > , # [doc = " The def_id and args of the current function."] pub (super) instance : ty :: Instance < 'tcx > , # [doc = " Extra data for the machine."] pub extra : Extra , # [doc = " Where to continue when returning from this function."] return_cont : ReturnContinuation , # [doc = " The location where the result of the current stack frame should be written to,"] # [doc = " and its layout in the caller. This place is to be interpreted relative to the"] # [doc = " *caller's* stack frame. We use a `PlaceTy` instead of an `MPlaceTy` since this"] # [doc = " avoids having to move *all* return places into Miri's memory."] pub return_place : PlaceTy < 'tcx , Prov > , # [doc = " The list of locals for this stack frame, stored in order as"] # [doc = " `[return_ptr, arguments..., variables..., temporaries...]`."] # [doc = " The locals are stored as `Option<Value>`s."] # [doc = " `None` represents a local that is currently dead, while a live local"] # [doc = " can either directly contain `Scalar` or refer to some part of an `Allocation`."] # [doc = ""] # [doc = " Do *not* access this directly; always go through the machine hook!"] pub locals : IndexVec < mir :: Local , LocalState < 'tcx , Prov > > , # [doc = " The span of the `tracing` crate is stored here."] # [doc = " When the guard is dropped, the span is exited. This gives us"] # [doc = " a full stack trace on all tracing statements."] tracing_span : SpanGuard , # [doc = " If this is `Right`, we are not currently executing any particular statement in"] # [doc = " this frame (can happen e.g. during frame initialization, and during unwinding on"] # [doc = " frames without cleanup code)."] # [doc = ""] # [doc = " Needs to be public because ConstProp does unspeakable things to it."] pub (super) loc : Either < mir :: Location , Span > , }
    };
}

Frame!();
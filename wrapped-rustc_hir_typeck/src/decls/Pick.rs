macro_rules! deps {
    () => {
        Candidate!();
        AutorefOrPtrAdjustment!();
        PickKind!();
    };
}

macro_rules! Pick {
    () => {
        deps!();
        # [derive (Debug , Clone)] pub (crate) struct Pick < 'tcx > { pub item : ty :: AssocItem , pub kind : PickKind < 'tcx > , pub import_ids : SmallVec < [LocalDefId ; 1] > , # [doc = " Indicates that the source expression should be autoderef'd N times"] # [doc = " ```ignore (not-rust)"] # [doc = " A = expr | *expr | **expr | ..."] # [doc = " ```"] pub autoderefs : usize , # [doc = " Indicates that we want to add an autoref (and maybe also unsize it), or if the receiver is"] # [doc = " `*mut T`, convert it to `*const T`."] pub autoref_or_ptr_adjustment : Option < AutorefOrPtrAdjustment > , pub self_ty : Ty < 'tcx > , # [doc = " Unstable candidates alongside the stable ones."] unstable_candidates : Vec < (Candidate < 'tcx > , Symbol) > , # [doc = " Number of jumps along the `Receiver::Target` chain we followed"] # [doc = " to identify this method. Used only for deshadowing errors."] # [doc = " Only applies for inherent impls."] pub receiver_steps : Option < usize > , # [doc = " Candidates that were shadowed by supertraits."] pub shadowed_candidates : Vec < ty :: AssocItem > , }
    };
}

Pick!()
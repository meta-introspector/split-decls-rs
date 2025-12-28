macro_rules! LateEarlyMismatch {
    () => {
        # [allow (unused)] enum LateEarlyMismatch < 'tcx > { EarlyInImpl (DefId , DefId , ty :: Region < 'tcx >) , LateInImpl (DefId , DefId , ty :: Region < 'tcx >) , }
    };
}

LateEarlyMismatch!();
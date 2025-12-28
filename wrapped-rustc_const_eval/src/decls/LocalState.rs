macro_rules! deps {
    () => {
        LocalValue!();
        State!();
    };
}

macro_rules! LocalState {
    () => {
        deps!();
        # [doc = " State of a local variable including a memoized layout"] # [derive (Clone)] pub struct LocalState < 'tcx , Prov : Provenance = CtfeProvenance > { value : LocalValue < Prov > , # [doc = " Don't modify if `Some`, this is only used to prevent computing the layout twice."] # [doc = " Avoids computing the layout of locals that are never actually initialized."] layout : Cell < Option < TyAndLayout < 'tcx > > > , }
    };
}

LocalState!();
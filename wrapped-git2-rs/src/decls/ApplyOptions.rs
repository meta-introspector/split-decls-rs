macro_rules! deps {
    () => {
        HunkCB!();
        DeltaCB!();
    };
}

macro_rules! ApplyOptions {
    () => {
        deps!();
        # [doc = " Options to specify when applying a diff"] pub struct ApplyOptions < 'cb > { raw : raw :: git_apply_options , hunk_cb : Option < Box < HunkCB < 'cb > > > , delta_cb : Option < Box < DeltaCB < 'cb > > > , }
    };
}

ApplyOptions!();
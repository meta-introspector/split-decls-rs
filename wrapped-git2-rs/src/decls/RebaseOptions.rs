macro_rules! deps {
    () => {
        MergeOptions!();
        CheckoutBuilder!();
        Rebase!();
    };
}

macro_rules! RebaseOptions {
    () => {
        deps!();
        # [doc = " Rebase options"] # [doc = ""] # [doc = " Use to tell the rebase machinery how to operate."] pub struct RebaseOptions < 'cb > { raw : raw :: git_rebase_options , rewrite_notes_ref : Option < CString > , merge_options : Option < MergeOptions > , checkout_options : Option < CheckoutBuilder < 'cb > > , }
    };
}

RebaseOptions!()
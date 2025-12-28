macro_rules! deps {
    () => {
        Rebase!();
    };
}

macro_rules! RebaseOperation {
    () => {
        deps!();
        # [doc = " A rebase operation"] # [doc = ""] # [doc = " Describes a single instruction/operation to be performed during the"] # [doc = " rebase."] # [derive (Debug)] pub struct RebaseOperation < 'rebase > { raw : * const raw :: git_rebase_operation , _marker : marker :: PhantomData < Rebase < 'rebase > > , }
    };
}

RebaseOperation!()
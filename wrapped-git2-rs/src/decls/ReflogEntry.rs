macro_rules! deps {
    () => {
        Reflog!();
    };
}

macro_rules! ReflogEntry {
    () => {
        deps!();
        # [doc = " An entry inside the reflog of a repository"] pub struct ReflogEntry < 'reflog > { raw : * const raw :: git_reflog_entry , _marker : marker :: PhantomData < & 'reflog Reflog > , }
    };
}

ReflogEntry!()
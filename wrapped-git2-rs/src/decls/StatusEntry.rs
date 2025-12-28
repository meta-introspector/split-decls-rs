macro_rules! deps {
    () => {
        Statuses!();
        DiffDelta!();
    };
}

macro_rules! StatusEntry {
    () => {
        deps!();
        # [doc = " A structure representing an entry in the `Statuses` structure."] # [doc = ""] # [doc = " Instances are created through the `.iter()` method or the `.get()` method."] pub struct StatusEntry < 'statuses > { raw : * const raw :: git_status_entry , _marker : marker :: PhantomData < & 'statuses DiffDelta < 'statuses > > , }
    };
}

StatusEntry!();
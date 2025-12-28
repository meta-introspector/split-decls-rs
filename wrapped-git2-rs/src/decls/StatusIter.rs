macro_rules! deps {
    () => {
        Statuses!();
    };
}

macro_rules! StatusIter {
    () => {
        deps!();
        # [doc = " An iterator over the statuses in a `Statuses` instance."] pub struct StatusIter < 'statuses > { statuses : & 'statuses Statuses < 'statuses > , range : Range < usize > , }
    };
}

StatusIter!()
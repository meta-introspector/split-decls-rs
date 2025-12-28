macro_rules! deps {
    () => {
        Entry!();
        VacantEntryImpl!();
    };
}

macro_rules! VacantEntry {
    () => {
        deps!();
        # [doc = " A vacant Entry. It is part of the [`Entry`] enum."] pub struct VacantEntry < 'a > { vacant : VacantEntryImpl < 'a > , }
    };
}

VacantEntry!()
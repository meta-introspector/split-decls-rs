macro_rules! deps {
    () => {
        ArenaMap!();
        Entry!();
    };
}

macro_rules! VacantEntry {
    () => {
        deps!();
        # [doc = " A view into an vacant entry in a [`ArenaMap`]. It is part of the [`Entry`] enum."] pub struct VacantEntry < 'a , IDX , V > { slot : & 'a mut Option < V > , _ty : PhantomData < IDX > , }
    };
}

VacantEntry!();
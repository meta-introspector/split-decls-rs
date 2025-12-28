macro_rules! deps {
    () => {
        ArenaMap!();
        Entry!();
    };
}

macro_rules! OccupiedEntry {
    () => {
        deps!();
        # [doc = " A view into an occupied entry in a [`ArenaMap`]. It is part of the [`Entry`] enum."] pub struct OccupiedEntry < 'a , IDX , V > { slot : & 'a mut Option < V > , _ty : PhantomData < IDX > , }
    };
}

OccupiedEntry!();
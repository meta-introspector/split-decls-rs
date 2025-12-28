macro_rules! deps {
    () => {
        LinearMapView!();
    };
}

macro_rules! OccupiedEntry {
    () => {
        deps!();
        # [doc = " An occupied entry which can be manipulated"] pub struct OccupiedEntry < 'a , K , V > { idx : usize , map : & 'a mut LinearMapView < K , V > , }
    };
}

OccupiedEntry!()
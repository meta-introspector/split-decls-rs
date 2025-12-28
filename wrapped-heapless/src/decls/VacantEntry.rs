macro_rules! deps {
    () => {
        LinearMapView!();
    };
}

macro_rules! VacantEntry {
    () => {
        deps!();
        # [doc = " A view into an empty slot in the underlying map"] pub struct VacantEntry < 'a , K , V > { key : K , map : & 'a mut LinearMapView < K , V > , }
    };
}

VacantEntry!();
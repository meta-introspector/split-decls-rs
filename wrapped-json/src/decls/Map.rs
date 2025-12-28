macro_rules! deps {
    () => {
        MapImpl!();
    };
}

macro_rules! Map {
    () => {
        deps!();
        # [doc = " Represents a JSON key/value type."] pub struct Map < K , V > { map : MapImpl < K , V > , }
    };
}

Map!()
macro_rules! deps {
    () => {
        OccupiedEntry!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl < 'a , IDX , V > OccupiedEntry < 'a , IDX , V > { # [doc = " Gets a reference to the value in the entry."] pub fn get (& self) -> & V { self . slot . as_ref () . expect ("Occupied") } # [doc = " Gets a mutable reference to the value in the entry."] pub fn get_mut (& mut self) -> & mut V { self . slot . as_mut () . expect ("Occupied") } # [doc = " Converts the entry into a mutable reference to its value."] pub fn into_mut (self) -> & 'a mut V { self . slot . as_mut () . expect ("Occupied") } # [doc = " Sets the value of the entry with the `OccupiedEntry`’s key, and returns the entry’s old value."] pub fn insert (& mut self , value : V) -> V { self . slot . replace (value) . expect ("Occupied") } # [doc = " Takes the value of the entry out of the map, and returns it."] pub fn remove (self) -> V { self . slot . take () . expect ("Occupied") } }
    };
}

impl_18!();
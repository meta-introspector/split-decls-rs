macro_rules! RichLocation {
    () => {
        # [derive (Copy , Clone , Debug)] pub enum RichLocation { Start (Location) , Mid (Location) , }
    };
}

RichLocation!();
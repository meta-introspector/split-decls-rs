macro_rules! FromDeserialized {
    () => {
        # [doc = " Map from deserialized representation"] pub trait FromDeserialized : Sized { type Input ; fn from_deserialized < E > (input : Self :: Input) -> Result < Self , E > where E : Error ; }
    };
}

FromDeserialized!()
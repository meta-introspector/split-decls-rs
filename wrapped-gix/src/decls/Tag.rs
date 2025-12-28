macro_rules! deps {
    () => {
        Clone!();
        Repository!();
    };
}

macro_rules! Tag {
    () => {
        deps!();
        # [doc = " A decoded tag object with access to its owning repository."] # [derive (Clone)] pub struct Tag < 'repo > { # [doc = " The id of the tree"] pub id : ObjectId , # [doc = " The fully decoded tag data"] pub data : Vec < u8 > , # [doc = " The owning repository."] pub repo : & 'repo Repository , }
    };
}

Tag!()
macro_rules! deps {
    () => {
        Repository!();
        Clone!();
        Kind!();
    };
}

macro_rules! Object {
    () => {
        deps!();
        # [doc = " A decoded object with a reference to its owning repository."] # [derive (Clone)] pub struct Object < 'repo > { # [doc = " The id of the object"] pub id : ObjectId , # [doc = " The kind of the object"] pub kind : gix_object :: Kind , # [doc = " The fully decoded object data"] pub data : Vec < u8 > , # [doc = " The owning repository."] pub repo : & 'repo Repository , }
    };
}

Object!()
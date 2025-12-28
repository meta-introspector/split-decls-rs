macro_rules! deps {
    () => {
        Repository!();
        Clone!();
    };
}

macro_rules! Blob {
    () => {
        deps!();
        # [doc = " A blob along with access to its owning repository."] # [derive (Clone)] pub struct Blob < 'repo > { # [doc = " The id of the tree"] pub id : ObjectId , # [doc = " The blob's data."] pub data : Vec < u8 > , # [doc = " The owning repository."] pub repo : & 'repo Repository , }
    };
}

Blob!();
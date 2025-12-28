macro_rules! deps {
    () => {
        Clone!();
        Repository!();
    };
}

macro_rules! Commit {
    () => {
        deps!();
        # [doc = " A decoded commit object with access to its owning repository."] # [derive (Clone)] pub struct Commit < 'repo > { # [doc = " The id of the commit"] pub id : ObjectId , # [doc = " The fully decoded commit data"] pub data : Vec < u8 > , # [doc = " The owning repository."] pub repo : & 'repo Repository , }
    };
}

Commit!()
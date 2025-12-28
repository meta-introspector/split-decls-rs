macro_rules! deps {
    () => {
        Repository!();
        Clone!();
    };
}

macro_rules! Id {
    () => {
        deps!();
        # [doc = " An [`ObjectId`] with access to a repository."] # [derive (Clone , Copy)] pub struct Id < 'r > { # [doc = " The actual object id"] pub (crate) inner : ObjectId , # [doc = " The owning repository."] pub repo : & 'r Repository , }
    };
}

Id!()
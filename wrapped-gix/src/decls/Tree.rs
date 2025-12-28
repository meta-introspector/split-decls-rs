macro_rules! deps {
    () => {
        Clone!();
        Repository!();
    };
}

macro_rules! Tree {
    () => {
        deps!();
        # [doc = " A decoded tree object with access to its owning repository."] # [derive (Clone)] pub struct Tree < 'repo > { # [doc = " Thek[ id of the tree"] pub id : ObjectId , # [doc = " The fully decoded tree data"] pub data : Vec < u8 > , # [doc = " The owning repository."] pub repo : & 'repo Repository , }
    };
}

Tree!();
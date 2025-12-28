macro_rules! deps {
    () => {
        File!();
        Options!();
    };
}

macro_rules! PackContext {
    () => {
        deps!();
        # [doc = " Information to allow verifying the integrity of an index with the help of its corresponding pack."] pub struct PackContext < 'a , F > { # [doc = " The pack data file itself."] pub data : & 'a crate :: data :: File , # [doc = " The options further configuring the pack traversal and verification"] pub options : integrity :: Options < F > , }
    };
}

PackContext!()
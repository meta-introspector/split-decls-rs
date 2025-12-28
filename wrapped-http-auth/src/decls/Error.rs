macro_rules! Error {
    () => {
        # [doc = " Describes a parse error and where in the input it occurs."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub struct Error < 'i > { input : & 'i str , pos : usize , error : & 'static str , }
    };
}

Error!()
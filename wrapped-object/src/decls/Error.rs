macro_rules! Error {
    () => {
        # [doc = " The error type used within the build module."] # [derive (Debug , Clone , PartialEq , Eq)] pub struct Error (pub (super) String) ;
    };
}

Error!()
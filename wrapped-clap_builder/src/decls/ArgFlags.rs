macro_rules! ArgFlags {
    () => {
        # [derive (Default , Copy , Clone , Debug , PartialEq , Eq)] pub (crate) struct ArgFlags (u32) ;
    };
}

ArgFlags!()
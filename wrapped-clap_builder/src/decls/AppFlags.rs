macro_rules! AppFlags {
    () => {
        # [derive (Default , Copy , Clone , Debug , PartialEq , Eq)] pub (crate) struct AppFlags (u32) ;
    };
}

AppFlags!()
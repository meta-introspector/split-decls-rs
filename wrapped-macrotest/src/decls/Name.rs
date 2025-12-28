macro_rules! Name {
    () => {
        # [derive (Serialize , Clone , Debug)] pub struct Name (pub String) ;
    };
}

Name!()
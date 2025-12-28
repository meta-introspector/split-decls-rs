macro_rules! TypeName {
    () => {
        # [derive (Copy , Clone , Debug , Hash , PartialEq , Eq)] pub struct TypeName (pub & 'static str , pub & 'static str) ;
    };
}

TypeName!()
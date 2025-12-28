macro_rules! DefPathDataName {
    () => {
        # [derive (Copy , Clone , PartialEq , Debug)] pub enum DefPathDataName { Named (Symbol) , Anon { namespace : Symbol } , }
    };
}

DefPathDataName!();
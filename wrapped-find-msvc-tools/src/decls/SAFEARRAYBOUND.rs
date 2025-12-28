macro_rules! SAFEARRAYBOUND {
    () => {
        # [repr (C)] # [derive (Clone , Copy , Default)] pub struct SAFEARRAYBOUND { pub cElements : u32 , pub lLbound : i32 , }
    };
}

SAFEARRAYBOUND!()
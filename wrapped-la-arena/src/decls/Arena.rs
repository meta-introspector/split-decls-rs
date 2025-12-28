macro_rules! Arena {
    () => {
        # [doc = " Yet another index-based arena."] # [derive (Clone , PartialEq , Eq , Hash)] pub struct Arena < T > { data : Vec < T > , }
    };
}

Arena!()
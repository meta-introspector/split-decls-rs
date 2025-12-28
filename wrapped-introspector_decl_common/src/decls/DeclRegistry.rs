macro_rules! deps {
    () => {
        DeclInfo!();
    };
}

macro_rules! DeclRegistry {
    () => {
        deps!();
        # [derive (Debug , Clone , Default)] pub struct DeclRegistry { pub declarations : Vec < DeclInfo > , pub by_type : HashMap < String , Vec < usize > > , pub by_module : HashMap < String , Vec < usize > > , pub by_hash : HashMap < String , usize > , }
    };
}

DeclRegistry!();
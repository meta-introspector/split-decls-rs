macro_rules! deps {
    () => {
        Value!();
        Type!();
    };
}

macro_rules! Constant {
    () => {
        deps!();
        # [derive (Copy , Clone)] pub struct Constant { pub Type : u8 , pub Parent : HasConstant , pub Value : id :: BlobId , }
    };
}

Constant!()
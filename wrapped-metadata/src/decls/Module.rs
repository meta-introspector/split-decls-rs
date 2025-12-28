macro_rules! Module {
    () => {
        # [derive (Default)] pub struct Module { pub Generation : u16 , pub Name : id :: StringId , pub Mvid : u32 , pub EncId : u32 , pub EncBaseId : u32 , }
    };
}

Module!()
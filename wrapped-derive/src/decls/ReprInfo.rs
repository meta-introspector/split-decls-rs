macro_rules! ReprInfo {
    () => {
        # [derive (Default)] pub struct ReprInfo { pub c : bool , pub transparent : bool , pub u8 : bool , pub packed : bool , }
    };
}

ReprInfo!()
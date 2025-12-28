macro_rules! ChangeIdConfig {
    () => {
        # [derive (Debug , Deserialize , Clone)] pub struct ChangeIdConfig { pub id : Option < String > , }
    };
}

ChangeIdConfig!()